# Soroban Crowdfunding Smart Contract

Ini adalah contoh implementasi smart contract untuk platform crowdfunding (penggalangan dana) yang dibangun menggunakan Rust dan Soroban untuk berjalan di atas blockchain Stellar.

Contract ini memungkinkan seorang kreator untuk membuat sebuah kampanye penggalangan dana dengan target (goal) dan batas waktu (deadline) tertentu. Para donatur dapat mengirimkan token XLM ke dalam contract. Jika target tidak tercapai hingga batas waktu berakhir, donatur dapat menarik kembali dana mereka.

## 🚀 Fitur Utama
- **Inisialisasi Kampanye**: Atur kreator, target dana, dan batas waktu.
- **Donasi**: Terima donasi dalam bentuk token XLM.
- **Pelacakan Progress**: Lihat total dana terkumpul dan persentasenya terhadap target.
- **Refund**: Izinkan donatur untuk menarik kembali dana jika target tidak tercapai setelah batas waktu.
- **Pengecekan Status**: Fungsi untuk melihat status kampanye (apakah sudah berakhir, apakah target tercapai).

---

## 🛠️ 1. Persiapan Lingkungan (Prerequisites)

Sebelum memulai, pastikan Anda telah menginstal perangkat berikut.

### a. Instalasi Rust
Jika belum terinstal, ikuti instruksi di situs resminya:
➡️ **[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)**

### b. Instalasi Soroban CLI
Ini adalah alat utama untuk membangun, mendeploy, dan berinteraksi dengan smart contract Soroban.

```sh
cargo install soroban-cli
```

Setelah instalasi, **tutup dan buka kembali terminal Anda**, lalu verifikasi dengan:
```sh
soroban --version
```
Anda seharusnya melihat nomor versi seperti `soroban-cli 20.x.x`.

<details>
<summary>⚠️ **Troubleshooting Instalasi Soroban CLI**</summary>

Jika Anda mengalami masalah saat instalasi, coba solusi berikut:

1.  **Error: `soroban: command not found`**
    Ini berarti lokasi instalasi `cargo` belum ada di `PATH` terminal Anda. Jalankan perintah ini, lalu tutup dan buka kembali terminal Anda.
    ```sh
    source "$HOME/.cargo/env"
    ```

2.  **Error: `binary 'stellar' already exists...`**
    Ini berarti ada konflik dengan versi lama (`stellar-cli`). Gunakan flag `--force` untuk menimpanya.
    ```sh
    cargo install soroban-cli --force
    ```

3.  **Error Kompilasi (`could not compile soroban-cli`)**
    Ini bisa terjadi karena *cache* atau versi yang tidak cocok. Solusinya adalah melakukan instalasi bersih:
    ```sh
    # 1. Uninstall versi yang mungkin sudah terinstal sebagian
    cargo uninstall soroban-cli

    # 2. Coba instal lagi
    cargo install soroban-cli
    ```
</details>

---

## ⚙️ 2. Alur Kerja Proyek

### a. Clone Repositori
```sh
# Ganti dengan URL repositori Anda jika perlu
git clone https://github.com/your-username/yopi-token.git
cd yopi-token/contracts/crowdfunding
```

### b. Jalankan Tes Lokal
Sebelum mendeploy, pastikan semua logika contract berjalan sesuai harapan dengan menjalankan tes lokal.
```sh
cargo test
```

### c. Build Kontrak
Kompilasi kode Rust menjadi file WebAssembly (.wasm) yang siap untuk di-deploy.
```sh
soroban contract build
```
File output akan berada di `../../target/wasm32-unknown-unknown/release/crowdfunding.wasm`.

---

## 🌐 3. Tutorial Deploy & Interaksi di Testnet

Tutorial ini menunjukkan cara mendeploy dan berinteraksi dengan contract di **Stellar Testnet**.

### a. Siapkan Akun Testnet
Anda memerlukan akun Testnet yang memiliki saldo XLM untuk membayar biaya transaksi.

```sh
# Buat identitas baru bernama 'user1' (nama bisa apa saja)
soroban config identity generate user1

# Dapatkan alamat publiknya (diawali dengan 'G...')
soroban config identity address user1

# Minta dana dari Testnet Friendbot. Buka link berikut di browser
# dan paste alamat 'G...' Anda untuk mendapatkan XLM gratis.
# -> https://friendbot.stellar.org/
```

### b. Deploy Kontrak
Deploy file `.wasm` yang sudah Anda build ke Testnet.

```sh
soroban contract deploy \
  --wasm ../../target/wasm32-unknown-unknown/release/crowdfunding.wasm \
  --source user1 \
  --network testnet
```
**PENTING**: Salin **Contract ID** (diawali dengan `C...`) yang muncul setelah deployment berhasil. Simpan ID ini.

### c. Inisialisasi Kampanye
Setelah di-deploy, contract harus diinisialisasi satu kali.

- **`--id`**: Contract ID dari langkah sebelumnya.
- **`--owner`**: Alamat yang akan menjadi pemilik kampanye (gunakan alamat `user1` Anda).
- **`--goal`**: Target dana dalam satuan stroops (1 XLM = 10,000,000 stroops). Contoh: 100 XLM = `1000000000`.
- **`--deadline`**: Batas waktu dalam format Unix timestamp.
- **`--xlm_token`**: Alamat contract token XLM di Testnet (ini adalah alamat tetap).

```sh
# Ganti <YOUR_CONTRACT_ID> dan <YOUR_ACCOUNT_ADDRESS>
# Contoh deadline: 24 jam dari sekarang
DEADLINE=$(($(date +%s) + 86400))
CONTRACT_ID="<YOUR_CONTRACT_ID>"
OWNER_ADDRESS="<YOUR_ACCOUNT_ADDRESS>"
XLM_TOKEN_ADDRESS="CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source user1 \
  --network testnet \
  --fn initialize \
  --arg "{\\"owner\\":\\"$OWNER_ADDRESS\\",\\"goal\\":1000000000,\\"deadline\\":$DEADLINE,\\"xlm_token\\":\\"$XLM_TOKEN_ADDRESS\\"}"
```

### d. Berdonasi ke Kampanye
Sekarang, siapa pun bisa berdonasi. Di sini, kita menggunakan `user1` sebagai donatur.

```sh
# Contoh donasi: 20 XLM = 200000000 stroops
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source user1 \
  --network testnet \
  --fn donate \
  --arg "{\\"donor\\":\\"$OWNER_ADDRESS\\",\\"amount\\":200000000}"
```

### e. Cek Status Kampanye
Gunakan fungsi "read-only" untuk melihat progress tanpa perlu mengirim transaksi.

```sh
# Cek total dana terkumpul
soroban contract read --id "$CONTRACT_ID" --fn get_total_raised

# Cek persentase progress
soroban contract read --id "$CONTRACT_ID" --fn get_progress_percentage
```

### f. Tarik Dana (Refund)
Jika batas waktu sudah lewat DAN target tidak tercapai, donatur bisa meminta refund.

```sh
# Pastikan Anda menjalankan ini setelah deadline terlewati
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source user1 \
  --network testnet \
  --fn refund \
  --arg "{\\"donor\\":\\"$OWNER_ADDRESS\\"}"
```
