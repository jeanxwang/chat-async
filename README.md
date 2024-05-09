# Tutorial 10 - Adpro

## Aliyah Faza Qinthara

### 2.1. Original code of broadcast chat.
![alt text](https://cdn.discordapp.com/attachments/1169297012211056731/1238067418945949758/Screenshot_2024-05-09_165353.png?ex=663defa2&is=663c9e22&hm=3d004d7c627502ccecea0208fe032dbdebadd9e7d2d4adf69c1215d48e5171d3&)

Setelah menjalankan server menggunakan perintah "cargo run --bin server" dan client-client menggunakan perintah "cargo run --bin client", hasilnya menunjukkan bahwa setiap client dan server menerima pesan broadcast dari setiap client lainnya. Saat seorang client mengirim pesan melalui command line, pesan tersebut akan dikirimkan ke server, yang kemudian akan meneruskannya ke semua client yang terhubung.

### 2.3. Small changes. Add some information to client
![alt text](https://cdn.discordapp.com/attachments/1169297012211056731/1238070381193986079/image.png?ex=663df265&is=663ca0e5&hm=fb98c8f09380677405bced7dd2afdee4d3f3549759fc412a434b77b3571d1ccd&)

Untuk menyertakan informasi tentang pengirim (identitas client) pada setiap pesan yang diterima, saya menambahkan identitas server ("Alin - Server") sebelum menampilkan pesan dari server.
