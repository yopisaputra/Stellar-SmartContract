# Soroban Crowdfunding Smart Contract

Ini adalah contoh implementasi smart contract untuk platform crowdfunding (penggalangan dana) yang dibangun menggunakan Rust dan Soroban untuk berjalan di atas blockchain Stellar.

Contract ini memungkinkan seorang kreator untuk membuat sebuah kampanye penggalangan dana dengan target (goal) dan batas waktu (deadline) tertentu. Para donatur dapat mengirimkan token XLM ke dalam contract. Jika target tidak tercapai hingga batas waktu berakhir, donatur dapat menarik kembali dana mereka.

## Fitur Utama
- **Inisialisasi Kampanye**: Atur kreator, target dana, dan batas waktu.
- **Donasi**: Terima donasi dalam bentuk token XLM.
- **Pelacakan Progress**: Lihat total dana terkumpul dan persentasenya terhadap target.
- **Refund Otomatis**: Izinkan donatur untuk menarik kembali dana jika target tidak tercapai setelah batas waktu.
- **Pengecekan Status**: Fungsi untuk melihat status kampanye (apakah sudah berakhir, apakah target tercapai).

---

## Prasyarat (Prerequisites)
Sebelum memulai, pastikan Anda telah menginstal perangkat berikut:

1.  **Rust Toolchain**: Ikuti instruksi di [rust-lang.org](https://www.rust-lang.org/tools/install).
2.  **Soroban CLI**: Alat command-line untuk berinteraksi dengan smart contract Soroban. Instal melalui `cargo`:
    ```sh
    cargo install soroban-cli
    ```

---

## Instalasi & Build

1.  **Clone Repositori**
    ```sh
    # Ganti dengan URL repositori Anda
    git clone https://github.com/your-username/yopi-token.git
    cd yopi-token
    ```

2.  **Build Kontrak**
    Perintah ini akan mengkompilasi kode Rust Anda menjadi file WebAssembly (.wasm) yang siap untuk di-deploy ke blockchain.
    ```sh
    soroban contract build
    ```
    File output akan berada di `target/wasm32-unknown-unknown/release/yopi_token.wasm`.

---

## Tutorial Penggunaan (via Soroban CLI)

Tutorial ini menunjukkan cara mendeploy dan berinteraksi dengan contract di **Stellar Testnet**.

### 1. Siapkan Identitas (Akun)
Anda memerlukan akun Testnet yang memiliki saldo XLM untuk membayar biaya transaksi.

```sh
# Buat identitas baru bernama 'user1'
soroban config identity generate user1

# Dapatkan alamatnya
soroban config identity address user1
# Output: G...

# Minta dana dari Testnet Friendbot (lakukan ini di browser)
# Buka: https://friendbot.stellar.org/ dan paste alamat G... Anda
```

### 2. Deploy Kontrak
Deploy file `.wasm` yang sudah Anda build ke Testnet.

```sh
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/yopi_token.wasm \
  --source user1 \
  --network testnet
```
**PENTING**: Salin **Contract ID** (diawali dengan `C...`) yang muncul setelah deployment berhasil.

### 3. Inisialisasi Kampanye
Setelah di-deploy, contract harus diinisialisasi.

- **`--id`**: Contract ID dari langkah sebelumnya.
- **`--owner`**: Alamat yang akan menjadi pemilik kampanye (bisa `user1`).
- **`--goal`**: Target dana dalam satuan stroops (1 XLM = 10,000,000 stroops). Contoh: 100 XLM = `1000000000`.
- **`--deadline`**: Batas waktu dalam format Unix timestamp. Anda bisa mendapatkan timestamp saat ini + durasi (misal: 24 jam = 86400 detik).
- **`--xlm_token`**: Alamat contract token XLM di Testnet (selalu sama).

```sh
# Ganti <CONTRACT_ID> dan <OWNER_ADDRESS>
# Contoh deadline: 24 jam dari sekarang
DEADLINE=$(($(date +%s) + 86400))

soroban contract invoke \
  --id <CONTRACT_ID> \
  --source user1 \
  --network testnet \
  --fn initialize \
  --arg-val "{\"owner\": \"<OWNER_ADDRESS>\", \"goal\": 1000000000, \"deadline\": $DEADLINE, \"xlm_token\": \"CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC\"}"
```

### 4. Berdonasi ke Kampanye
Sekarang, siapa pun bisa berdonasi.

```sh
# Ganti <CONTRACT_ID> dan <DONOR_ADDRESS>
# Contoh donasi: 20 XLM = 200000000
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source user1 \
  --network testnet \
  --fn donate \
  --arg-val "{\"donor\": \"<DONOR_ADDRESS>\", \"amount\": 200000000}"
```

### 5. Cek Status Kampanye
Gunakan fungsi "read-only" untuk melihat progress.

```sh
# Cek total dana terkumpul
soroban contract read \
  --id <CONTRACT_ID> \
  --fn get_total_raised

# Cek persentase progress
soroban contract read \
  --id <CONTRACT_ID> \
  --fn get_progress_percentage
```

### 6. Tarik Dana (Refund)
Jika batas waktu sudah lewat DAN target tidak tercapai, donatur bisa meminta refund.

```sh
# Ganti <CONTRACT_ID> dan <DONOR_ADDRESS>
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source user1 \
  --network testnet \
  --fn refund \
  --arg-val "{\"donor\": \"<DONOR_ADDRESS>\"}"
```

---

## Menjalankan Tes Lokal
Untuk memverifikasi semua fungsi berjalan sesuai ekspektasi secara lokal, jalankan:
```sh
cargo test
```
