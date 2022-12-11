fn main() {

    //-----------------------------Wikipedia(Rekursion)-------------------------------------------------

    {
        let mut fibonatchizahlenkette0 = vec![];                // Vektor für wikipedia berechnung
        for fibzahl in 0..44{                                   // 44 durchgänge, weil ab 45 Durchgang zu lange Laufzeit 
            fibonatchizahlenkette0.push(fibonacci0(fibzahl));    // hinzufügen der Zahl die mit der Funktion fibbonacci (wikipedia) berechnet wird
        }
        println!("fibonatchizahlenkette = {:?}",fibonatchizahlenkette0);    // ausgabe aller berechneten Zahlen
    }    
    
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

