module Lyh where

firstLetter :: String -> String
firstLetter "" = "Empty string, whoops!"
firstLetter a@(x:_) = "The first Letter of " ++ a ++ " is " ++ [x]

myCompare
  :: (Ord a)
  => a -> a -> Ordering
a `myCompare` b
  | a == b = EQ
  | a <= b = LT
  | otherwise = GT

bmiTell :: Double -> Double -> String
bmiTell weight height
  | bmi <= skinny = "You're underweight, you emo, you!"
  | bmi <= normal = "You're supposedly normal.Pffft, I bet you're ugly!"
  | bmi <= fat = "You're fat! Lost some weight, fatty!"
  | otherwise = "You're a whale, congratulations!"
  where
    bmi = weight / height ^ 2
    skinny = 18.5
    normal = 25.0
    fat = 30.0

--(skinny, normal, fat) = (18.5, 25.0, 30.0)
--global variable
niceGreeting :: String
niceGreeting = "Hello! So very nice to see you,"

badGreeting :: String
badGreeting = "Oh! Pffft.It's you."

greet :: String -> String
greet "Juan" = niceGreeting ++ " Juan!"
greet "Fernando" = niceGreeting ++ " Fernando!"
greet name = badGreeting ++ " " ++ name

initials :: String -> String -> String
initials firstname lastname = [f] ++ ". " ++ [l] ++ "."
  where
    (f:_) = firstname
    (l:_) = lastname

calcBmis :: [(Double, Double)] -> [Double]
calcBmis xs = [bmi w h | (w, h) <- xs]
  where
    bmi weight height = weight / height ^ 2

cylinder :: Double -> Double -> Double
cylinder r h =
  let sideArea = 2 * pi * r * h
      topArea = pi * r ^ 2
  in sideArea + 2 * topArea

fac :: Integer -> Integer
fac 0 = 1
fac n
  | n > 0 = n * fac (n - 1)
--ghci> 4 * (let a = 9 in a + 1) + 2
--42
--ghci> [let square x = x * x in (square 5, square 3, square 2)]
--[(25,9,4)]
--ghci> (let (a,b,c) = (1,2,3) in a+b+c) * 100
--600
