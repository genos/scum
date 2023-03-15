{-# LANGUAGE DerivingVia #-}
{-# LANGUAGE OverloadedStrings #-}

module Scum.Expression (Expr (..), Identifier (..)) where

import Data.String (IsString)
import Data.Text (Text)
import qualified Data.Text as T

newtype Identifier = Identifier Text
    deriving (Eq, Ord, IsString) via Text

instance Show Identifier where
    show (Identifier i) = T.unpack i

data Expr
    = Atom Identifier
    | Bool Bool
    | Float Double
    | Int Integer
    | List [Expr]
    deriving (Eq, Ord)

instance Show Expr where
    show = T.unpack . toText

tshow :: Show a => a -> Text
tshow = T.pack . show

paren :: (a -> Text) -> [a] -> Text
paren f xs = T.concat ["(", T.unwords $ f <$> xs, ")"]

toText :: Expr -> Text
toText (Atom a) = tshow a
toText (Bool b) = if b then "#t" else "#f"
toText (Float x) = T.pack $ show x
toText (Int n) = T.pack $ show n
toText (List xs) = paren toText xs
