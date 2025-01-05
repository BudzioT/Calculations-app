# Aplikacja do Szacowania Ceny Usług

## Przegląd

Ta aplikacja jest zaprojektowana, aby pomóc firmom oszacować koszt ich usług na podstawie różnych parametrów, takich jak liczba dni, odległość i liczba pracowników. Zapewnia przyjazny interfejs użytkownika do dodawania usług, ich wyboru i obliczania całkowitej ceny.

## Dedykowani Użytkownicy

- **Właściciele Firm**: Do oszacowania kosztu usług świadczonych klientom.
- **Kierownicy Projektów**: Do planowania i budżetowania wydatków związanych z usługami.
- **Zespoły Sprzedażowe**: Do dostarczania dokładnych wycen potencjalnym klientom.

## Przypadki Użycia

1. **Dodawanie Usługi**: Użytkownicy mogą dodawać nowe usługi z określonymi szczegółami, takimi jak nazwa, cena za dzień, podatek, koszt dojazdu, limit odległości, dodatkowa opłata za dojazd, limit pracowników i dodatkowe dni.
2. **Wybór Usługi**: Użytkownicy mogą wybrać usługę z listy rozwijanej do wykonania obliczeń.
3. **Obliczanie Ceny**: Użytkownicy mogą wprowadzić liczbę dni, odległość i liczbę pracowników, aby obliczyć całkowitą cenę wybranej usługi.

## Algorytm Funkcjonowania

1. **Dodawanie Usługi**: Użytkownicy wprowadzają szczegóły usługi, które są przechowywane na liście.
2. **Wybór Usługi**: Użytkownicy wybierają usługę z listy do obliczenia ceny.
3. **Obliczanie Ceny**: Aplikacja oblicza całkowitą cenę na podstawie parametrów wybranej usługi i danych wprowadzonych przez użytkownika (dni, odległość, pracownicy).

## Charakterystyka Aplikacji

### Zabezpieczenia

- **Walidacja Danych**: Zapewnia, że wszystkie dane wprowadzone przez użytkownika są prawidłowe i mieszczą się w oczekiwanych zakresach.
- **Obsługa Błędów**: Dostarcza jasne komunikaty o błędach dla nieprawidłowych operacji.

### Przechowywanie Danych

- **Przechowywanie w Pamięci**: Usługi i dane wprowadzone przez użytkownika są przechowywane w pamięci podczas sesji. Nie jest używane trwałe przechowywanie.

### Wymagania Systemowe

- **System Operacyjny**: Windows
- **Rust**: Najnowsza stabilna wersja
- **Node.js i npm**: Do Tailwind CSS
- **Dioxus**: Do budowania interfejsu użytkownika

## Instrukcje Użytkowania

### Wymagania Wstępne

1. **Zainstaluj Rust**: Postępuj zgodnie z instrukcjami na [rust-lang.org](https://www.rust-lang.org/tools/install).
2. **Zainstaluj Node.js i npm**: Postępuj zgodnie z instrukcjami na [nodejs.org](https://nodejs.org/).

### Konfiguracja

1. **Sklonuj Repozytorium**:
    ```bash
    git clone <repository-url>
    cd <repository-directory>
    ```

2. **Zainstaluj Tailwind CSS**:
    ```bash
    npm install -D tailwindcss
    npx tailwindcss init
    ```

3. **Uruchom Kompilator Tailwind CSS**:
    ```bash
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
    ```

### Uruchamianie Aplikacji

1. **Uruchom Aplikację**:
    ```bash
    dx serve --platform desktop
    ```

2. **Dostęp do Aplikacji**:
   Otwórz aplikację w przeglądarce lub środowisku desktopowym zgodnie z poleceniem `dx serve`.

## Podsumowanie

Ta aplikacja zapewnia solidne rozwiązanie do szacowania kosztów usług, co czyni ją niezbędnym narzędziem dla firm do efektywnego zarządzania i planowania wydatków związanych z usługami.