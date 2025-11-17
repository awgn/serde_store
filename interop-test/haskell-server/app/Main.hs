{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE ScopedTypeVariables #-}

module Main (main) where

import Web.Scotty
import Network.Wai.Middleware.RequestLogger (logStdoutDev)
import Network.HTTP.Types.Status (status200, status400, status500)
import Data.Store (Store, encode, decode)
import qualified Data.ByteString as BS
import qualified Data.ByteString.Lazy as LBS
import qualified Data.Text.Lazy as TL
import Control.Monad.IO.Class (liftIO)
import Data.Word (Word8)

import Types

main :: IO ()
main = do
    putStrLn "================================================"
    putStrLn "Haskell Store Echo Server"
    putStrLn "================================================"
    putStrLn "Starting on port 3000..."
    putStrLn ""
    putStrLn "Endpoints:"
    putStrLn "  POST /echo/primitives   - Echo TestPrimitives"
    putStrLn "  POST /echo/strings      - Echo TestStrings"
    putStrLn "  POST /echo/collections  - Echo TestCollections"
    putStrLn "  POST /echo/nested       - Echo TestNested"
    putStrLn "  POST /echo/enum         - Echo TestEnum"
    putStrLn "  POST /echo/tuples       - Echo TestTuples"
    putStrLn "  POST /echo/either       - Echo TestEither"
    putStrLn "  POST /echo              - Echo with type tag"
    putStrLn "  GET  /health            - Health check"
    putStrLn ""
    putStrLn "Ready to accept connections..."
    putStrLn "================================================"
    
    scotty 3000 $ do
        middleware logStdoutDev
        
        -- Health check
        get "/health" $ do
            text "OK"
        
        -- Echo endpoints for each type
        post "/echo/primitives" $ echoHandler (Proxy :: Proxy TestPrimitives) "primitives"
        post "/echo/strings" $ echoHandler (Proxy :: Proxy TestStrings) "strings"
        post "/echo/collections" $ echoHandler (Proxy :: Proxy TestCollections) "collections"
        post "/echo/nested" $ echoHandler (Proxy :: Proxy TestNested) "nested"
        post "/echo/enum" $ echoHandler (Proxy :: Proxy TestEnum) "enum"
        post "/echo/tuples" $ echoHandler (Proxy :: Proxy TestTuples) "tuples"
        post "/echo/either" $ echoHandler (Proxy :: Proxy TestEither) "either"
        
        -- Generic echo endpoint with type tag (first byte is type discriminator)
        post "/echo" $ do
            bodyBytes <- body
            let bs = LBS.toStrict bodyBytes
            
            liftIO $ putStrLn $ "\n[/echo] Received " ++ show (BS.length bs) ++ " bytes"
            
            if BS.null bs
                then do
                    status status400
                    text "Empty request body"
                else case decodeRequest bs of
                    Left err -> do
                        liftIO $ putStrLn $ "[/echo] ERROR: " ++ err
                        status status400
                        text $ TL.pack err
                    Right (typeTag, payload) -> do
                        liftIO $ putStrLn $ "[/echo] Type tag: " ++ show typeTag
                        response <- liftIO $ processPayload typeTag payload
                        case response of
                            Left err -> do
                                liftIO $ putStrLn $ "[/echo] Processing ERROR: " ++ err
                                status status500
                                text $ TL.pack err
                            Right respBytes -> do
                                liftIO $ putStrLn $ "[/echo] SUCCESS: Returning " ++ show (BS.length respBytes) ++ " bytes"
                                status status200
                                setHeader "Content-Type" "application/octet-stream"
                                raw $ LBS.fromStrict respBytes

-- Proxy type for type-level programming
data Proxy a = Proxy

-- Type-specific echo handler
echoHandler :: forall a. (Store a, Show a, Eq a) => Proxy a -> String -> ActionM ()
echoHandler _ typeName = do
    bodyBytes <- body
    let bs = LBS.toStrict bodyBytes
    
    liftIO $ putStrLn $ "\n[" ++ typeName ++ "] Received " ++ show (BS.length bs) ++ " bytes"
    
    case decode bs of
        Left err -> do
            liftIO $ putStrLn $ "[" ++ typeName ++ "] Decode ERROR: " ++ show err
            status status500
            text $ TL.pack $ "Decode error: " ++ show err
        Right (val :: a) -> do
            liftIO $ putStrLn $ "[" ++ typeName ++ "] Decoded successfully: " ++ show val
            let encoded = encode val
            liftIO $ putStrLn $ "[" ++ typeName ++ "] Re-encoded to " ++ show (BS.length encoded) ++ " bytes"
            
            -- Verify roundtrip
            case decode encoded of
                Left err -> do
                    liftIO $ putStrLn $ "[" ++ typeName ++ "] ROUNDTRIP FAILED: " ++ show err
                    status status500
                    text $ TL.pack $ "Roundtrip verification failed: " ++ show err
                Right (val2 :: a) -> do
                    if val == val2
                        then do
                            liftIO $ putStrLn $ "[" ++ typeName ++ "] Roundtrip verified successfully"
                            status status200
                            setHeader "Content-Type" "application/octet-stream"
                            raw $ LBS.fromStrict encoded
                        else do
                            liftIO $ putStrLn $ "[" ++ typeName ++ "] ROUNDTRIP VALUE MISMATCH!"
                            status status500
                            text "Roundtrip value mismatch"

-- Decode request with type tag (first byte is type, rest is payload)
decodeRequest :: BS.ByteString -> Either String (Word8, BS.ByteString)
decodeRequest bs
    | BS.length bs < 1 = Left "Request too short (need at least 1 byte for type tag)"
    | otherwise = 
        let typeTag = BS.head bs
            payload = BS.tail bs
        in Right (typeTag, payload)

-- Process payload based on type tag
processPayload :: Word8 -> BS.ByteString -> IO (Either String BS.ByteString)
processPayload typeTag payload = do
    putStrLn $ "Processing type " ++ show typeTag ++ " with " ++ show (BS.length payload) ++ " bytes"
    case typeTag of
        1 -> echoTyped (Proxy :: Proxy TestPrimitives) "primitives" payload
        2 -> echoTyped (Proxy :: Proxy TestStrings) "strings" payload
        3 -> echoTyped (Proxy :: Proxy TestCollections) "collections" payload
        4 -> echoTyped (Proxy :: Proxy TestNested) "nested" payload
        5 -> echoTyped (Proxy :: Proxy TestEnum) "enum" payload
        6 -> echoTyped (Proxy :: Proxy TestTuples) "tuples" payload
        7 -> echoTyped (Proxy :: Proxy TestEither) "either" payload
        _ -> return $ Left $ "Unknown type tag: " ++ show typeTag

-- Generic typed echo
echoTyped :: forall a. (Store a, Show a, Eq a) => Proxy a -> String -> BS.ByteString -> IO (Either String BS.ByteString)
echoTyped _ typeName bs = do
    case decode bs of
        Left err -> do
            putStrLn $ "[" ++ typeName ++ "] Decode error: " ++ show err
            return $ Left $ "Decode error: " ++ show err
        Right (val :: a) -> do
            putStrLn $ "[" ++ typeName ++ "] Decoded: " ++ show val
            let encoded = encode val
            putStrLn $ "[" ++ typeName ++ "] Re-encoded to " ++ show (BS.length encoded) ++ " bytes"
            return $ Right encoded