mod fibonacci;

use crate::ergebnisgenerator::fibonacci::fibonacci;

pub fn ergebnisgenerator(ende:u32) -> Vec<u32> {

    //-------------------------------------Fibonaccizahlenkette------------------------------------------
    
    let mut fibonaccikette: Vec<u128> = vec![0,1];                         // erstelle Vektor mit: erste bedingung f(0) = 0 und zweite bedingung f(1) = 1
        for fibzahl in 2..ende{
  
            fibonaccikette.push(fibonacci(fibzahl as usize,&fibonaccikette));       // da Fibonaccizahlen sowie exponentialzahlen schnell groß werden: Umwandlung in u128,wechsel in Fibonacci Funktion 
        }    

    //----------------------------------------------------------------------------------------------------


    //-------------------------------------Exponentialzahlenkette-----------------------------------------
    
    let mut exponentialkette: Vec<u128> = vec![1;6];                        // um anstieg auszubremsen initialisierung mit 6x 1
        for x in 1..ende-1{                                                 // x = 0 überspringen 
            let x: f64 = x as f64;                                          // konvertierung in f32 da powf kein u
            let x = f64::powf(1.57,x);                                      // 1,57^a um die Zahl klein zu halten
            exponentialkette.push(x as u128);                               // in exponentialkette einfügen
        }

    //println!("exponentialkette = {:?}",exponentialkette);

    //----------------------------------------------------------------------------------------------------


    //-------------------------------------Ergebniszahlenkette-------------------------------------------
    
    let mut ergebniskette:Vec<u32> = vec![];

        for x in 0..ende{
            let x = x as usize;                                             // indexelemente müssen usize sein
            let ergebnis= fibonaccikette[x]/exponentialkette[x];            // 
            let ergebnis= ergebnis as u32;                                  // umwandlung zurück in kleineren Typ
            ergebniskette.push(ergebnis);
        }
        //println!("ergebniskette = {:?}",ergebniskette);
        //println!("elemente im ergebniss = {:?}",ergebniskette.len())
        ergebniskette                                                              // gib ergebniskette zurück
}

