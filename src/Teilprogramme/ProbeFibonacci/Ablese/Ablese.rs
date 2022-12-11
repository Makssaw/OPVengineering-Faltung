fn main() {

    //------------------------------Ableseberechnung------------------------------------------------
    
        let mut fibonatchizahlenkette: Vec<u128> = vec![];     // Vektor für eigene berechnung 
        fibonatchizahlenkette.push(0);                         // erste Bedingung f(0) = 0
        fibonatchizahlenkette.push(1);                         // zweite Bedingung f(1) = 1

        for fibzahl in 2..100{
            fibonatchizahlenkette.push(fibonacci1(fibzahl,&fibonatchizahlenkette));
        }
        println!("fibonatchizahlenkette = {:?}",fibonatchizahlenkette);    // ausgabe aller berechneten Zahlen

    //----------------------------------------------------------------------------------------------

}


//----------------------------------------------------------------------------------------------
fn fibonacci1(fibzahl:u128,fibonatchizahlenkette: &Vec<u128>) -> u128{     // Vektorelementablese

    let fibzahl_usize: usize = fibzahl as usize;                            // Vektorelemente müssen usize sein 
            let fibzahl= fibonatchizahlenkette[fibzahl_usize-2]
                        +fibonatchizahlenkette[fibzahl_usize-1];           // Formel ab x > 2: f(x) = f(x-1) + f(x-2)
            fibzahl
 }
//----------------------------------------------------------------------------------------------

