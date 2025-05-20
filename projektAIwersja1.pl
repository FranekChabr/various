% I
kobieta(zofia_kowal).
kobieta(genowefa_kowal).
% II
kobieta(bozena_kowal).
kobieta(zofia_kowalska).
% III
kobieta(maria_tkacz).
kobieta(dorota_kowal).
kobieta(danuta_kowalska).
kobieta(irena_zieba).
% IV
kobieta(dzesika_tkacz).
kobieta(julia_zieba).
%==============================================================================%
% I
mezczyzna(stanislaw_kowal).
% II
mezczyzna(roman_kowal).
mezczyzna(stanislaw_kowalski).
mezczyzna(jan_kowal).
mezczyzna(wojciech_mokrogorski).
% III
mezczyzna(arkadiusz_tkacz).
mezczyzna(krzysztof_kowal).
mezczyzna(lech_kowalski).
% IV
mezczyzna(sebastian_tkacz).
mezczyzna(pawel_kowal).
mezczyzna(grzegorz_kowalski).
mezczyzna(marek_kowalski).
% V
mezczyzna(brajan_tkacz).

%==============================================================================%

% I->II
rodzic(zofia_kowal, roman_kowal).
rodzic(stanislaw_kowal, roman_kowal).
rodzic(genowefa_kowal, jan_kowal).
rodzic(stanislaw_kowal, jan_kowal).
rodzic(genowefa_kowal, wojciech_mokrogorski).

% II->III
rodzic(bozena_kowal, maria_tkacz).          % maria
rodzic(roman_kowal, maria_tkacz).           % maria
rodzic(bozena_kowal, krzysztof_kowal).      % krzysiek
rodzic(roman_kowal, krzysztof_kowal).       % krzysiek
rodzic(zofia_kowalska, dorota_kowal).       % dorota
rodzic(stanislaw_kowalski, dorota_kowal).   % dorota
rodzic(zofia_kowalska, lech_kowalski).      % leszek
rodzic(stanislaw_kowalski, lech_kowalski).  % leszek
rodzic(zofia_kowalska, irena_zieba).        % irena

% III->IV
rodzic(maria_tkacz, sebastian_tkacz).        % seba
rodzic(arkadiusz_tkacz, sebastian_tkacz).    % seba
rodzic(dorota_kowal, pawel_kowal).           % pawel
rodzic(krzysztof_kowal, pawel_kowal).        % pawel 
rodzic(danuta_kowalska, grzegorz_kowalski).  % grzesiek
rodzic(lech_kowalski, grzegorz_kowalski).    % grzesiek
rodzic(danuta_kowalska, marek_kowalski).     % marek
rodzic(lech_kowalski, marek_kowalski).       % marek
rodzic(irena_zieba, julia_zieba).			 % julcia 
% IV->V
rodzic(dzesika_tkacz, brajan_tkacz).
rodzic(sebastian_tkacz, brajan_tkacz).

%==============================================================================%
% I
malzenstwo(genowefa_kowal, stanislaw_kowal).
malzenstwo(zofia_kowal, stanislaw_kowal).
% II
malzenstwo(bozena_kowal, roman_kowal).
malzenstwo(zofia_kowalska, stanislaw_kowalski).
% III
malzenstwo(maria_tkacz, arkadiusz_tkacz).
malzenstwo(dorota_kowal, krzysztof_kowal).
malzenstwo(danuta_kowalska, lech_kowalski).
% IV
malzenstwo(dzesika_tkacz, sebastian_tkacz).

%malzenstwo(X,Y) :- malzenstwo_f(X,Y) ; malzenstwo_f(Y,X).  % przemiennosc 

%==============================================================================%

matka(X,Y) :- kobieta(X), rodzic(X, Y).
ojciec(X,Y) :- mezczyzna(X), rodzic(X, Y).
corka(X,Y) :- kobieta(X), rodzic(Y, X).
syn(X,Y) :- mezczyzna(X), rodzic(Y, X).
babcia(X,Z) :- kobieta(X), rodzic(X, Y), rodzic(Y, Z). 
dziadek(X,Z) :- mezczyzna(X), rodzic(X, Y), rodzic(Y, Z). 
wnuczka(X,Z) :- kobieta(X), rodzic(Z, Y), rodzic(Y, X).
wnuczek(X,Z) :- mezczyzna(X), rodzic(Z, Y), rodzic(Y, X).
prababcia(X,W) :- kobieta(X), rodzic(X, Y), rodzic(Y, Z), rodzic(Z, W).    % W jak wnuczek :DDD
pradziadek(X,W) :- mezczyzna(X), rodzic(X, Y), rodzic(Y, Z), rodzic(Z, W). %

przodek(X,Y) :- rodzic(X, Y) ; rodzic(X, A), rodzic(A, Y) ; rodzic(X, A), rodzic(A, B), rodzic(B, Y).
potomek(X,Y) :- rodzic(Y, X) ; rodzic(Y, A), rodzic(A, X) ; rodzic(Y, A), rodzic(A, B), rodzic(B, X).

siostra_rodzona(X,Y) :- kobieta(X), 
    matka(M,X), matka(M,Y),
    ojciec(O,X), ojciec(O,Y),
    X \= Y.
brat_rodzony(X,Y) :- mezczyzna(X), 
    matka(M,X), matka(M,Y),
    ojciec(O,X), ojciec(O,Y),
    X \= Y.

rodzenstwo(X,Y) :- siostra_rodzona(X,Y) ; brat_rodzony(X,Y) ; siostra_przyrodnia(X,Y) ; brat_przyrodni(X,Y).
siostra(X,Y) :- siostra_rodzona(X,Y) ; siostra_przyrodnia(X,Y).  % oba na raz
brat(X,Y) :- brat_rodzony(X,Y) ; brat_przyrodni(X,Y). 			 % oba na raz

% % % % % % % % % % % % % % % % 

siostra_przyrodnia(X,Y) :- kobieta(X), 
    rodzic(Rodzic1, X), rodzic(Rodzic1, Y), \+ (rodzic(Rodzic2, X), rodzic(Rodzic2, Y), 
	Rodzic1 \= Rodzic2), 
    X \= Y. 

brat_przyrodni(X,Y) :- mezczyzna(X), 
    rodzic(Rodzic1, X), rodzic(Rodzic1, Y), \+ (rodzic(Rodzic2, X), rodzic(Rodzic2, Y), 
	Rodzic1 \= Rodzic2), 
    X \= Y. 
    
%==============================================================================%

kuzynka(X,Y) :- kobieta(X), rodzic(Z,Y), rodzenstwo(Z,P), corka(X,P).
kuzyn(X,Y) :- mezczyzna(X), rodzic(Z,Y), rodzenstwo(Z,P), syn(X,P).
ciocia(X,Y) :- kobieta(X), (rodzic(Z,Y), rodzenstwo(Z,X) ; malzenstwo(X,P), brat(P,Z), rodzic(Z,Y)).
wujek(X,Y) :- mezczyzna(X), (matka(Z,Y), rodzenstwo(Z,X)) ; malzenstwo(X,P), siostra(P,Z), rodzic(Z,Y).
stryj(X,Y) :- mezczyzna(X), ojciec(Z,Y), brat(Z,X).
stryjenka(X,Y) :- kobieta(X), stryj(Z,Y), malzenstwo(Z,X).

%==============================================================================%

szwagier(X,Y) :- mezczyzna(X), (brat(X, Maz), malzenstwo(Y, Maz)) ; 
    (malzenstwo(Siostra, X), siostra(Siostra, Y)) ; 
    (brat(X, Zona),malzenstwo(Zona, Y)). 
% X jest mezczyzna, X jest bratem meza Y   lub
% X jest mężem siostry Y 				   lub
% X jest bratem zony Y

szwagierka(X,Y) :- kobieta(X), (siostra(X, Maz), malzenstwo(Y, Maz)) ;
    (siostra(X, Zona), malzenstwo(Zona, Y)) ;
    (malzenstwo(X, Brat), brat(Brat, Y)) ;
    (malzenstwo(Zona, Y), brat(Brat, Zona), malzenstwo(X, Brat)).
% X jest kobieta, X jest siostra meza Y    lub
% X jest siostra zony Y                    lub
% X jest zona brata Y                      lub
% X jest zona brata zony Y

macocha(X,Y) :- kobieta(X), malzenstwo(X,Z), ojciec(Z,Y), \+ rodzic(X,Y).
ojczym(X,Y) :- mezczyzna(X), malzenstwo(Z,X), matka(Z,Y), \+ rodzic(X,Y).
pasierb(X,Y) :- mezczyzna(X), (malzenstwo(Y,Z), syn(X,Z), \+ syn(X,Y)) ; (malzenstwo(Z,Y), syn(X,Z), \+ syn(X,Y)).
pasierbica(X,Y) :- kobieta(X), rodzic(Rodzic, X), malzenstwo(Rodzic, Y), \+ rodzic(Y, X).

%==============================================================================%

babcia_od_strony_matki(X,Y) :- matka(X, M), matka(M, Y).   
babcia_od_strony_ojca(X,Y) :- matka(X, M), ojciec(M, Y). 
dziadek_od_strony_matki(X,Y) :- ojciec(X, M), matka(M, Y). 
dziadek_od_strony_ojca(X,Y) :- ojciec(X, M), ojciec(M, Y).   

% wymyslanki: =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

syn_stryja(X,Y) :- stryj(Z,Y), syn(X,Z).
przyszywana_babka(X,Y) :- malzenstwo(X,Z), dziadek(Z,Y), \+ babcia(X,Y).
wujek_wujka(X,Y) :- wujek(X,Z), wujek(Z,Y). 
wujek_szwagra(X,Y) :- wujek(X,Z), szwagier(Z,Y). 
zona_brata_zony(X,Y) :- malzenstwo(X,Z), brat(Z,P), malzenstwo(P,Y). 
przyszywany_wujek(X,Y) :- brat_przyrodni(X,M), matka(M,Y).

szwagier_brat_meza(X,Y) :- szwagier(X,Y), brat(X,Z), malzenstwo(Y,Z). 
szwagier_brat_zony(X,Y) :- szwagier(X,Y), brat(X,Z), malzenstwo(Z,Y). 
zona_kuzyna(X,Y) :- malzenstwo(X,Z), kuzyn(Z,Y). 
maz_kuzynki(X,Y) :- malzenstwo(Z,X), kuzynka(Z,Y).
szwagier_ojca(X,Y) :- szwagier(X,Z), ojciec(Z,Y).
brat_dziadka(X,Y) :- brat(X,Z), dziadek(Z,Y).
syn_corki(X,Y) :- syn(X,Z), corka(Z,Y).
siostra_syna(X,Y) :- siostra(X,Z), syn(Z,Y).
stary_stryj_kawaler(X,Y) :- stryj(X,Y), \+ rodzic(X,_). 

tesc(X,Y) :- ojciec(X,Z), malzenstwo(Z,Y). %ojciec zony

ciocia_stara_panna(X,Y) :- ciocia(X,Y), \+ rodzic(X,_), \+ malzenstwo(X,_). 



