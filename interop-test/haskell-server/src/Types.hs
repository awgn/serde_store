{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}

module Types
    ( TestPrimitives(..)
    , TestStrings(..)
    , TestCollections(..)
    , TestNested(..)
    , TestEnum(..)
    , TestTuples(..)
    , TestEither(..)
    , Person(..)
    , Company(..)
    , TestRequest(..)
    , TestResponse(..)
    ) where

import Data.Store (Store)
import Data.Text (Text)
import Data.Map.Strict (Map)
import Data.Word
import Data.Int
import GHC.Generics (Generic)

-- | Test basic primitive types
data TestPrimitives = TestPrimitives
    { tpBool :: Bool
    , tpU8 :: Word8
    , tpU16 :: Word16
    , tpU32 :: Word32
    , tpU64 :: Word64
    , tpI8 :: Int8
    , tpI16 :: Int16
    , tpI32 :: Int32
    , tpI64 :: Int64
    , tpF32 :: Float
    , tpF64 :: Double
    } deriving (Generic, Show, Eq)

instance Store TestPrimitives

-- | Test string types
data TestStrings = TestStrings
    { tsString :: Text
    , tsEmpty :: Text
    , tsUnicode :: Text
    } deriving (Generic, Show, Eq)

instance Store TestStrings

-- | Test collections
data TestCollections = TestCollections
    { tcList :: [Int32]
    , tcMap :: Map Text Int32
    , tcEmpty :: [Text]
    } deriving (Generic, Show, Eq)

instance Store TestCollections

-- | Test nested structures
data Person = Person
    { personName :: Text
    , personAge :: Word32
    , personEmail :: Maybe Text
    } deriving (Generic, Show, Eq)

instance Store Person

data Company = Company
    { companyName :: Text
    , companyEmployees :: [Person]
    , companyRevenue :: Double
    } deriving (Generic, Show, Eq)

instance Store Company

data TestNested = TestNested
    { tnPerson :: Person
    , tnCompany :: Company
    } deriving (Generic, Show, Eq)

instance Store TestNested

-- | Test enums
data TestEnum
    = VariantA
    | VariantB Int32
    | VariantC { vcX :: Int32, vcY :: Int32 }
    deriving (Generic, Show, Eq)

instance Store TestEnum

-- | Test tuples of various sizes (2-7 elements, matching Haskell Store support)
data TestTuples = TestTuples
    { ttTuple2 :: (Int32, Text)
    , ttTuple3 :: (Word32, Text, Double)
    , ttTuple4 :: (Bool, Bool, Word32, Text)
    , ttTuple5 :: (Word32, Int32, Float, Text, Bool)
    , ttTuple6 :: (Word32, Int32, Float, Text, Bool, Bool)
    , ttTuple7 :: (Word32, Int32, Float, Text, Bool, Bool, Word64)
    , ttNested :: ((Word32, Word32), (Word32, Word32))
    , ttWithList :: ([Int32], Maybe Text, [Word32])
    } deriving (Generic, Show, Eq)

instance Store TestTuples

-- | Request/Response wrapper for the echo protocol
data TestRequest = TestRequest
    { reqType :: Word8  -- Type discriminator
    , reqData :: [Word8]  -- Binary payload
    } deriving (Generic, Show, Eq)

instance Store TestRequest

-- | Test Either type (similar to Rust's Result but more generic)
data TestEither = TestEither
    { teLeftInt :: Either Int32 Text
    , teRightString :: Either Bool Text
    , teNested :: Either (Either Int32 Text) Bool
    , teWithOption :: Either (Maybe Int32) Text
    , teWithList :: Either [Int32] Text
    } deriving (Generic, Show, Eq)

instance Store TestEither

data TestResponse = TestResponse
    { respType :: Word8
    , respData :: [Word8]
    , respSuccess :: Bool
    } deriving (Generic, Show, Eq)

instance Store TestResponse