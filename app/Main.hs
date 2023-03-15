module Main where

import qualified Scum (someFunc)

main :: IO ()
main = do
  putStrLn "Hello, Haskell!"
  Scum.someFunc
