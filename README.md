# RETCrypt - Güvenli Şifreleme Projesi

RETCrypt, Auralis takımının Teknofest Projesidir. Bu proje geleneksel simetrik şifrelemenin dışında kanal yapısı ile veriyi daha güvenli getirir

> [!NOTE]
> Sadece kanal ve şifreleme içerir. Hash doğrulaması şuan dahil değildir.
## Gelecek Planları

- ShiftRows ve MixColumns gibi algoritmalar eklenerek güvenlik seviyesinin artırılması.
- Kullanıcı dostu bir arayüz geliştirilmesi.
- Performans iyileştirmeleri ve hata düzeltmeleri.


> [!NOTE]
> Rust programlama dili ve cargo yüklenmelidir
### Kurulum
1. İşletim sisteminize göre rust dilini kurun
2. Repoyu kopyalayın
   ```sh
   git clone https://github.com/RecepEfe/RETCrypt.git
   ```
3. Dizine girin
   ```sh
   cd RETCrypt/
   ```
4. Kodu derleyin
   ```sh
   cargo build
   ```
5. Derlenmiş kodu çalıştırın
   ```sh
   cargo run
   ```
