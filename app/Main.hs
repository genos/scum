{-# LANGUAGE OverloadedStrings #-}
module Main where

import Scum (Expr(..))

main :: IO ()
main = print $ List [Atom "a", Atom "b", Atom "c"]
