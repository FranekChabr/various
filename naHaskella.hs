-- zad 1 

sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs

main :: IO ()                        -- testowanie
main = print (sumList [10, 20, 30])  -- wynik: 60


-- zad 2 ------------------------------------------------------------------------

main :: IO ()
main = do
    print kombinacje

zywioly :: [String]
zywioly = ["ogień", "woda", "powietrze", "ziemia"]

kombinacje :: [(String, String)]
kombinacje = pary zywioly

pary :: [String] -> [(String, String)]
pary [] = []
pary (x:xs) = [(x, y) | y <- xs] ++ pary xs

-- zad 3 --------------------------------------------------------------------------

