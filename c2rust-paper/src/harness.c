/* Kompilieren mit:
   gcc harness_save.c -std=c99 -O3 -o harness_save -lm
*/

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h> /* Für Dateinamen-Operationen benötigt */

/* Implementierungen aktivieren */
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

#define QOI_IMPLEMENTATION
#include "qoi.h"

#define ITERATIONS 100

int main(int argc, char **argv) {
    if (argc < 2) {
        printf("Nutzung: %s <bild_datei.png>\n", argv[0]);
        return 1;
    }

    const char *input_filename = argv[1];
    int w, h, channels;

    // 1. Original-Bild laden
    unsigned char *original_pixels = stbi_load(input_filename, &w, &h, &channels, 0);

    if (!original_pixels) {
        printf("Fehler: Konnte Bild '%s' nicht laden.\n", input_filename);
        return 1;
    }

    printf("Bild geladen: %dx%d, Kanäle: %d\n", w, h, channels);

    qoi_desc desc;
    desc.width = w;
    desc.height = h;
    desc.channels = channels;
    desc.colorspace = QOI_SRGB;

    int encoded_size = 0;
    void *encoded_data = NULL;
    // WICHTIG: Dieser Pointer wird das Ergebnis der letzten Runde halten
    void *final_decoded_pixels = NULL;

    printf("Starte Benchmark (%d Iterationen Encode + Decode)...\n", ITERATIONS);

    clock_t start_time = clock();

    for (int i = 0; i < ITERATIONS; i++) {
        // ENCODE
        encoded_data = qoi_encode(original_pixels, &desc, &encoded_size);
        if (!encoded_data) {
            printf("Fehler beim Encoding in Iteration %d\n", i);
            free(original_pixels); return 1;
        }

        // DECODE
        final_decoded_pixels = qoi_decode(encoded_data, encoded_size, &desc, channels);
        if (!final_decoded_pixels) {
            printf("Fehler beim Decoding in Iteration %d\n", i);
            free(encoded_data); free(original_pixels); return 1;
        }

        // Encoded Daten brauchen wir nicht mehr
        free(encoded_data);

        // WICHTIG: Wenn dies NICHT die letzte Iteration ist, geben wir den
        // Speicher sofort wieder frei, um kein RAM-Leck während des Benchmarks zu haben.
        if (i < (ITERATIONS - 1)) {
            free(final_decoded_pixels);
            final_decoded_pixels = NULL; // Sicherheitshalber nullen
        }
        // Wenn i == 99 ist, wird free() übersprungen und final_decoded_pixels
        // zeigt auf die Daten der letzten Runde.
    }

    clock_t end_time = clock();
    double time_spent = (double)(end_time - start_time) / CLOCKS_PER_SEC;

    printf("----------------------------------------\n");
    printf("Benchmark Fertig.\n");
    printf("Gesamtzeit:    %.4f Sekunden\n", time_spent);
    printf("Pro Iteration: %.4f ms\n", (time_spent * 1000.0) / ITERATIONS);
    printf("FPS (Enc+Dec): %.2f\n", (double)ITERATIONS / time_spent);

    // --- Ergebnis der letzten Iteration speichern ---

    if (final_decoded_pixels) {
        // Ausgabedateinamen erstellen (z.B. input.png -> input.png_qoi_result.png)
        char output_filename[256];
        snprintf(output_filename, sizeof(output_filename), "%s_qoi_result.png", input_filename);

        printf("----------------------------------------\n");
        printf("Speichere Ergebnis der letzten Iteration als: %s\n", output_filename);

        // Das stride (Bytes pro Zeile) ist Breite * Kanäle
        int stride_in_bytes = w * channels;

        int write_result = stbi_write_png(output_filename, w, h, channels, final_decoded_pixels, stride_in_bytes);

        if (write_result == 0) {
            printf("FEHLER beim Speichern des Ausgabebildes!\n");
        } else {
            printf("Erfolgreich gespeichert. Bitte visuell mit dem Original vergleichen.\n");
        }

        // JETZT den Speicher der letzten Iteration freigeben
        free(final_decoded_pixels);
    }

    // Original-Bild freigeben
    stbi_image_free(original_pixels);

    return 0;
}