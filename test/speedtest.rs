// Geschwindigkeitsvergleich Test zwischen Rekursion und Ablesen anhand der Fobonacciberechnung


fn main() {

    use std::time::Instant;                                     //https://doc.rust-lang.org/std/time/struct.Instant.html

    //-----------------------------Wikipedia(Rekursion)-------------------------------------------------

    let now = Instant::now();                                   //https://doc.rust-lang.org/std/time/struct.Instant.html

    {
        let mut fibonatchizahlenkette0 = vec![];                // Vektor für wikipedia berechnung
        for fibzahl in 0..44{                                   // 44 durchgänge, weil ab 45 Durchgang zu lange Laufzeit 
            fibonatchizahlenkette0.push(fibonacci0(fibzahl));    // hinzufügen der Zahl die mit der Funktion fibbonacci (wikipedia) berechnet wird
        }
        //println!("fibonatchizahlenkette = {:?}",fibonatchizahlenkette0);    // ausgabe aller berechneten Zahlen
    }    

    let elapsed = now.elapsed();                                //https://doc.rust-lang.org/std/time/struct.Instant.html
    println!("Dauer Wikipediaberechnung: {:.2?}", elapsed);     //

    //----------------------------------------------------------------------------------------------


    //------------------------------Ableseberechnung------------------------------------------------
    
    let now1 = Instant::now();
    {
        let mut fibonatchizahlenkette1: Vec<u128> = vec![];     // Vektor für eigene berechnung 
        fibonatchizahlenkette1.push(0);                         // erste Bedingung f(0) = 0
        fibonatchizahlenkette1.push(1);                         // zweite Bedingung f(1) = 1

        for fibzahl in 2..100{
            fibonatchizahlenkette1.push(fibonacci1(fibzahl,&fibonatchizahlenkette1));
        }

    } 
    let elapsed = now1.elapsed();
    println!("Dauer Ableseberechnung: {:.2?}", elapsed);

    //----------------------------------------------------------------------------------------------

}



//----------------------------------------------------------------------------------------------
fn fibonacci0(fibzahl:u32) -> u32{                                          // Rekursion

    if fibzahl == 0 {
        return 0;
    }
    else if fibzahl == 1 {
        return 1;
    }
    fibonacci0(fibzahl-1)+fibonacci0(fibzahl-2)                             // Formel ab x > 2: f(x) = f(x-1) + f(x-2)
}
//----------------------------------------------------------------------------------------------


//----------------------------------------------------------------------------------------------
fn fibonacci1(fibzahl:u128,fibonatchizahlenkette1: &Vec<u128>) -> u128{     // Vektorelementablese

    let fibzahl_usize: usize = fibzahl as usize;                            // Vektorelemente müssen usize sein 
            let fibzahl= fibonatchizahlenkette1[fibzahl_usize-2]
                        +fibonatchizahlenkette1[fibzahl_usize-1];           // Formel ab x > 2: f(x) = f(x-1) + f(x-2)
            fibzahl
 }
//----------------------------------------------------------------------------------------------



