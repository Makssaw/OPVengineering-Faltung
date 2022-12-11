//Autor: Maksymilian
#![allow(non_snake_case)]                                               //Fehler ausblenden: crate `` should have a snake case name

mod primzahlgenerator;                                                  // Zugriff auf extern abgelegte Datei: primzahlgenerator.rs
mod ergebnisgenerator;                                                  // Zugriff auf extern abgelegte Datei: ergebnisgenerator.rs 

use std::io;                                                            // Zugriff auf Eingabe
use crate::primzahlgenerator::primzahlgenerator;                        // Zugriff auf primzahlgenerator Funktion in primzahlgenerator.rs
use crate::ergebnisgenerator::ergebnisgenerator;                        // Zugriff auf ergebnisgenerator Funktion in ergebnisgenerator.rs



fn main() {

    //--------------------------------------Eingabe-------------------------------------------
    println!("Bitte zulässige Zahl an Punkten angeben (bis 187) die gefaltet werden sollen");  // ab 188: overflow der Fibonacci zahl

    let mut ende:String = String::new();                     // https://doc.rust-lang.org/stable/std/io/struct.Stdin.html
        io::stdin()                                         // Eingabe         
        .read_line(&mut ende)           // Einlese
        .expect("Zahl kann nicht gelesen werden");          // falls falsche Eingabe
    
    let ende: u32 = ende
        .trim()                                             // leerzeichen ignorieren
        .parse()                      // umwandlung String in integer
        .expect("Zahl ist kein zulässiger Integer");        // falls falsche Eingabe
    
    if ende > 187{
        println!("Zahl zu groß");
        std::process::exit(0);                          // wenn zahl zu groß Code beenden
    }

    println!("Faltung wird durchgeführt mit {ende} Punkten");
    //-----------------------------------------------------------------------------------------------


    //-------------------------------------Primzahlgenerator-----------------------------------------
     
    let primzahlenkette = primzahlgenerator(ende);              // rufe Primzahlgenerator Funktion auf und übergebe eingegebene zahl und speicher Vektor im main
        
    
    //println!("primzahlenkette = {:?}",primzahlenkette);
    //println!("elemente im ergebniss = {:?}",primzahlenkette.len());

    //-------------------------------------------------------------------------------------------------


    //------------------------------Ergebnisgenerator aus Fibonaccizahl/1,5^x---------------------------
    
    let ergebniskette = ergebnisgenerator(ende);                // rufe Ergebnisgenerator Funktion auf und übergebe eingegebene zahl und speicher Vektor im main
    //println!("ergebniskette = {:?}",ergebniskette);u


    //----------------------------------------Faltung Primzahlen und Ergebnis aus Fibonaccizahl/1,5^x -------------------------------------------------------
    let ende = ende as usize;                                       // Ab hier werden Variablen(x,r): usize 
    let mut faltungskette:Vec<u32>= vec![0;ende];                          // Initialisiere mit Nullern
    //let ende = ende as u32;
    
    for x in 0..ende{                                               // https://de.wikipedia.org/wiki/Faltung_(Mathematik)
 
        for r in 0..x{              
            faltungskette[x] += primzahlenkette[r] * ergebniskette[x-r];      
        }
    }
    println!("Die Faltung ergibt: {:?}",faltungskette);                     // Ausgabe der Faltung


}




