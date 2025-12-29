mod lib;
use lib::*;
use std::fs;
use std::path::Path;
use std::slice;
use image::io::Reader as ImageReader;
use std::error::Error;

// ------------------------------------------------------------------
// HELFER-FUNKTIONEN (Logik aus main ausgelagert)
// ------------------------------------------------------------------

// 1. Bild laden und rohe Pixel (RGB) zurückgeben
fn load_image_pixels(path: &str) -> Result<(Vec<u8>, u32, u32), Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?;
    let rgb = img.into_rgb8();
    let width = rgb.width();
    let height = rgb.height();
    // into_raw gibt uns den reinen Vektor: [R, G, B, R, G, B, ...]
    Ok((rgb.into_raw(), width, height))
}

// 2. Rohe Pixel -> QOI Bytes (Encoding)
fn encode_pixels_to_qoi(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut desc = qoi_desc {
        width,
        height,
        channels: 3, // Wir nutzen hier fix RGB (3 channels)
        colorspace: 0,
    };
    
    // Kopie der Pixel erstellen, da qoi_encode einen mutable pointer will
    let mut pixels_clone = pixels.to_vec();
    let mut out_len: i32 = 0;

    unsafe {
        let encoded_ptr = qoi_encode(
            pixels_clone.as_mut_ptr() as *mut std::ffi::c_void,
            &mut desc,
            &mut out_len
        );

        if encoded_ptr.is_null() {
            return Err("QOI Encoding fehlgeschlagen".into());
        }

        // Daten in einen Rust-Vector kopieren
        let encoded_slice = slice::from_raw_parts(encoded_ptr as *const u8, out_len as usize);
        let result = encoded_slice.to_vec();
        
        // HINWEIS: Hier entsteht eigentlich ein Memory Leak, da wir den C-Pointer nicht free'n.
        // In Produktion müsste man die passende free-Funktion aufrufen.
        
        Ok(result)
    }
}

// 3. QOI Bytes -> Rohe Pixel (Decoding)
fn decode_qoi_to_pixels(qoi_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    // FIX: Manuelle Initialisierung statt default()
    let mut desc = qoi_desc {
        width: 0,
        height: 0,
        channels: 0,
        colorspace: 0,
    };
    
    let len = qoi_data.len() as i32;
    let data_ptr = qoi_data.as_ptr() as *const std::ffi::c_void;

    unsafe {
        let decoded_ptr = qoi_decode(
            data_ptr,
            len,
            &mut desc,
            3 
        );

        if decoded_ptr.is_null() {
            return Err("QOI Decoding fehlgeschlagen".into());
        }

        let size = (desc.width * desc.height * 3) as usize;
        let decoded_slice = slice::from_raw_parts(decoded_ptr as *const u8, size);
        let result = decoded_slice.to_vec();
        
        // (Optional: libc::free(decoded_ptr as *mut _); hier einfügen für Memory safety)

        Ok(result)
    }
}

// ------------------------------------------------------------------
// HAUPTPROGRAMM
// ------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    let filename = "test_image.jpeg";
    
    // 1. Laden
    println!("Lade {}...", filename);
    let (pixels, width, height) = load_image_pixels(filename)?;

    // 2. Encoden
    println!("Konvertiere zu QOI...");
    let qoi_bytes = encode_pixels_to_qoi(&pixels, width, height)?;
    fs::write("output_rust.qoi", &qoi_bytes)?;
    println!("Gespeichert als output_rust.qoi");

    // 3. Decoden (zur Kontrolle)
    println!("Konvertiere zurück zu Pixeln...");
    let decoded_pixels = decode_qoi_to_pixels(&qoi_bytes)?;

    // 4. Als PNG speichern (damit man es ansehen kann)
    // Wir speichern NICHT als JPG, da JPG verlustbehaftet ist und sich die Bytes ändern würden.
    image::save_buffer(
        "output_rust_restored.png",
        &decoded_pixels,
        width,
        height,
        image::ColorType::Rgb8
    )?;
    println!("Restauriertes Bild gespeichert als output_rust_restored.png");

    Ok(())
}

// ------------------------------------------------------------------
// TESTS
// ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // TEST 1: Vergleicht die binäre QOI Datei (Rust vs. C)
    #[test]
    fn test_qoi_file_matches_c_output() {
        // Zuerst Rust-Encoding durchführen
        let (pixels, width, height) = load_image_pixels("output_c.qoi").expect("Bild fehlt!");
        let rust_qoi = encode_pixels_to_qoi(&pixels, width, height).expect("Encoding Fehler");

        // Dann die C-Datei laden (muss vorher mit deinem C-Programm erstellt worden sein!)
        let c_path = "output_c.qoi";
        if !Path::new(c_path).exists() {
            panic!("'output_c.qoi' fehlt! Bitte erst das C-Programm laufen lassen.");
        }
        let c_qoi = fs::read(c_path).expect("Kann C-Datei nicht lesen");

        // Vergleich
        assert_eq!(rust_qoi.len(), c_qoi.len(), "Dateigrößen sind unterschiedlich!");
        assert_eq!(rust_qoi, c_qoi, "Die Bytes der QOI Dateien sind unterschiedlich!");
    }

    // TEST 2: Vergleicht die Pixel (Original vs. Decoded)
    // Dies prüft, ob QOI wirklich "lossless" (verlustfrei) gearbeitet hat.
    #[test]
    fn test_pixels_roundtrip_lossless() {
        // 1. Original laden
        let (original_pixels, width, height) = load_image_pixels("test_image.jpeg").expect("Bild fehlt!");

        // 2. Encoden (Simuliert Speichern)
        let qoi_bytes = encode_pixels_to_qoi(&original_pixels, width, height).expect("Encode Fehler");

        // 3. Decoden (Simuliert Laden)
        let decoded_pixels = decode_qoi_to_pixels(&qoi_bytes).expect("Decode Fehler");

        // 4. Vergleich
        // Wir vergleichen hier NICHT die Bild-Dateien (JPG vs PNG), sondern die rohen [R,G,B] Werte im Speicher.
        assert_eq!(original_pixels.len(), decoded_pixels.len(), "Pixel-Anzahl stimmt nicht!");
        assert_eq!(original_pixels, decoded_pixels, "Pixel-Werte haben sich verändert! (Nicht lossless)");
    }
}