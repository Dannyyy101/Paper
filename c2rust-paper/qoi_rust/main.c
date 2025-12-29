#include <stdio.h>
#include <stdlib.h>

// 1. Definitionen für die Bibliotheken aktivieren
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

#define QOI_IMPLEMENTATION
#include "qoi.h"

int main() {
    const char *input_file = "test_image.jpeg";
    const char *output_file = "output_c.qoi";
    
    int w, h, channels;

    // 2. Bild laden
    // Der letzte Parameter '3' erzwingt RGB (3 Kanäle).
    // Das ist WICHTIG, damit es mit Rusts .into_rgb8() übereinstimmt.
    unsigned char *pixels = stbi_load(input_file, &w, &h, &channels, 3);

    if (!pixels) {
        printf("Fehler: Konnte %s nicht laden.\n", input_file);
        return 1;
    }

    printf("C: Bild geladen: %dx%d, 3 Channel.\n", w, h);

    // 3. QOI Description vorbereiten
    qoi_desc desc;
    desc.width = w;
    desc.height = h;
    desc.channels = 3; 
    desc.colorspace = QOI_SRGB;

    // 4. Encoden
    int out_len = 0;
    void *encoded = qoi_encode(pixels, &desc, &out_len);

    if (!encoded) {
        printf("Fehler beim Encoding.\n");
        stbi_image_free(pixels);
        return 1;
    }

    // 5. Datei schreiben
    FILE *f = fopen(output_file, "wb");
    if (!f) {
        printf("Fehler: Konnte %s nicht erstellen.\n", output_file);
        free(encoded);
        stbi_image_free(pixels);
        return 1;
    }

    fwrite(encoded, 1, out_len, f);
    fclose(f);

    printf("C: Datei geschrieben: %s (%d bytes)\n", output_file, out_len);

    // 6. Aufräumen
    free(encoded);          // QOI Speicher freigeben
    stbi_image_free(pixels); // Bild Speicher freigeben

    return 0;
}