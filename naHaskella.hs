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

data Kolor = Zielony | Czerwony | Niebieski deriving (Show, Eq)

wszystkieKolory :: [Kolor]
wszystkieKolory = [Zielony, Czerwony, Niebieski]

data Wierzcholek = A | B | C | D | E deriving (Show, Eq, Ord)

wszystkieWierzcholki :: [Wierzcholek]
wszystkieWierzcholki = [A, B, C, D, E]

type Krawedz = (Wierzcholek, Wierzcholek)

krawedzieGrafu :: [Krawedz]
krawedzieGrafu =
    [(A, B),
     (B, D),
     (A, C),
     (C, E),
     (D, E)]

type Kolorowanie = [(Wierzcholek, Kolor)]

pobierzKolor :: Wierzcholek -> Kolorowanie -> Maybe Kolor
pobierzKolor wierzcholek kolorowanie = lookup wierzcholek kolorowanie

czyPoprawne :: Kolorowanie -> Bool
czyPoprawne kolorowanie = all sprawdzKrawedz krawedzieGrafu
  where
    sprawdzKrawedz (u, v) =
        case (pobierzKolor u kolorowanie, pobierzKolor v kolorowanie) of
            (Just kolorU, Just kolorV) -> kolorU /= kolorV
            _                          -> False

generujKolorowania :: [Wierzcholek] -> [Kolor] -> [Kolorowanie]
generujKolorowania [] _ = [[]]
generujKolorowania (w:ws) kolory =
    [ (w, k) : reszta
    | k <- kolory
    , reszta <- generujKolorowania ws kolory
    ]

znajdzPoprawneKolorowania :: [Kolorowanie]
znajdzPoprawneKolorowania = filter czyPoprawne (generujKolorowania wszystkieWierzcholki wszystkieKolory)

wydrukujKolorowanie :: Kolorowanie -> IO ()
wydrukujKolorowanie kolorowanie = do
    putStrLn "--- Znaleziono poprawne kolorowanie ---"
    mapM_ (\(w, k) -> putStrLn $ show w ++ ": " ++ show k) kolorowanie
    putStrLn "----------------------------"

main :: IO ()
main = do
    let poprawneRozwiazania = znajdzPoprawneKolorowania
    if null poprawneRozwiazania
        then putStrLn "Nie znaleziono poprawnych kolorowań."
        else do
            putStrLn $ "Znaleziono " ++ show (length poprawneRozwiazania) ++ " poprawne kolorowanie(a):"
            mapM_ wydrukujKolorowanie poprawneRozwiazania
