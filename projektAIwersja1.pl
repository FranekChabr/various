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
malzenstwo_f(genowefa_kowal, stanislaw_kowal).
malzenstwo_f(zofia_kowal, stanislaw_kowal).
% II
malzenstwo_f(bozena_kowal, roman_kowal).
malzenstwo_f(zofia_kowalska, stanislaw_kowalski).
% III
malzenstwo_f(maria_tkacz, arkadiusz_tkacz).
malzenstwo_f(dorota_kowal, krzysztof_kowal).
malzenstwo_f(danuta_kowalska, lech_kowalski).
% IV
malzenstwo_f(dzesika_tkacz, sebastian_tkacz).

malzenstwo(X,Y) :- malzenstwo_f(X,Y) ; malzenstwo_f(Y,X).  

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

% % % % % % % % % % % % % % % % 

siostra_przyrodnia(X,Y) :- kobieta(X), 
    matka(M,X), matka(M,Y) ;
    ojciec(O,X), ojciec(O,Y),			% zle
    X \= Y.

brat_przyrodni(X,Y) :- mezczyzna(X), 
    matka(M,X), matka(M,Y) ;
    ojciec(O,X), ojciec(O,Y),		   % zle
    X \= Y.

%==============================================================================%











