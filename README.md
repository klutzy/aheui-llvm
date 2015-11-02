aheui-llvm
========
이 프로그램은 아희 코드를 llvm IR로 변환해주는 컴파일러 프론트엔드입니다.
Rust 언어로 짜여져 있으며, 예제 런타임 라이브러리 역시 Rust로 짜여져 있습니다.

### 요구사항
* rustc 1.5.0-nightly

### 사용법
1.  aheui-llvm을 실행하여, 아희 소스코드의 LLVM IR을 얻습니다.
    ```
    cargo run -- README.md
    ```

1.  llc로 이를 빌드하여 object 파일을 얻습니다.
    ```
    llc -O=3 -filetype=obj -relocation-model=pic README.md.ll
    ```

1.  아희 런타임과 object 파일을 링크하여 실행파일을 얻습니다.
    ```
    rustc -o hello runtime.rs -C opt-level=3 -C link-args="README.md.o"

    ./hello
    # Hello, world!
    ```

```
밤밣따빠밣밟따뿌
빠맣파빨받밤뚜뭏
돋밬탕빠맣붏두붇
볻뫃박발뚷투뭏붖
뫃도뫃희멓뭏뭏붘
뫃봌토범더벌뿌뚜
뽑뽀멓멓더벓뻐뚠
뽀덩벐멓뻐덕더벅
```
