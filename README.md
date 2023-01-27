<p align="center">
    <img src="images/logo.png" width=138>
</p>
<h1 align="center">Fultonic Entertainment</h1>
<p align="center"><strong>A thought-out, efficient <i>to-be</i> mass media web platform for our content</strong></p>
<div align="center" style="margin-bottom: .5em;">
    <a href="LICENSE"><img src="badges/agpl.svg" width="128" /></a>
</div>

### **Languages:**

![JS](https://img.shields.io/badge/JavaScript-F7DF1E?style=for-the-badge&logo=javascript&logoColor=black)
![HTML](https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white)
![CSS](https://img.shields.io/badge/CSS3-1572B6?style=for-the-badge&logo=css3&logoColor=white)
![WA](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)

<sup>No Javascript frameworks (upcoming Rust-only full-stack with WebAssembly)!</sup>

### **Tools:**

[![Askama](badges/askama.svg)](https://github.com/djc/askama)

> "(Not) yet another streaming service" [<img src="https://raw.githubusercontent.com/twitter/twemoji/master/assets/svg/1f921.svg" width=12 />](https://youtube.com/watch?v=e64_1DmbgN4)

<div>A thought-out, unparalleled content platform with mint focus on efficiency and performance.</div>
A website to symbolize our unique identity.

## *Efficient?*

Our website has been *uniquely* developed on the pursuit of efficiency with some Rust libraries, and without any Javascript frameworks, compared to most modern web development, as many learn developers that way <sup>(*why?* [1][1], [2][2], [3][3]) a, b</sup>

> <sup>(a) "Modern websites are highly complex and evolve over time in a path-dependent way, sometimes accumulating out-of-date features and code". In other words, the modern web is bloated with unnecessary features. <sup>[4][4] [(taken from 5)][5]</sup></sup>

> <sup>(b) The source of this rationale is rooted on an academic paper I've been writing <sup>[(me)](https://github.com/LeCodingWolfie)</sup> for and with another fellow researcher <sup>[(him)](https://github.com/)</sup> that thoroughly analyses the poor efficiency and *bloat* of modern software and the web, the consequences, and its causes, and how to fight against it, which everyone should know about (both developers and end users), with the title "Bloat, The Devil in The Dark". <sup>[(inspiration)](https://www.imdb.com/title/tt0708460/)</sup></sup>

Words are the least. Currently, the Rust program both generates the HTML code of the layout of our content (within ~100-200 LOC); then, with templates, builds the HTML for each page. At the end, everything runs under ~2-5ms (esp. in a `Release` build).

<details>
  <summary>Builds</summary>

  ### `Debug`

  ```
  $ cargo build
    Compiling iso v0.1.0 (/home/lewolfie/Files/Projects/ISO)
    Finished dev [unoptimized + debuginfo] target(s) in 1.10s
  $ perf stat -r 10 -d target/debug/iso
    Performance counter stats for 'target/debug/iso' (10 runs):

                6.95 msec task-clock:u                     #    0.890 CPUs utilized            ( +-  3.48% )
                   0      context-switches:u               #    0.000 /sec
                   0      cpu-migrations:u                 #    0.000 /sec
                 104      page-faults:u                    #   14.135 K/sec                    ( +-  0.33% )
           8,348,925      cycles:u                         #    1.135 GHz                      ( +-  8.18% )  (42.85%)
          16,232,185      instructions:u                   #    1.54  insn per cycle           ( +-  4.42% )  (57.05%)
           2,606,057      branches:u                       #  354.205 M/sec                    ( +-  2.61% )  (57.00%)
              27,491      branch-misses:u                  #    1.06% of all branches          ( +- 27.73% )  (56.88%)
           6,554,975      L1-dcache-loads:u                #  890.926 M/sec                    ( +-  3.33% )  (68.16%)
               8,844      L1-dcache-load-misses:u          #    0.14% of all L1-dcache accesses  ( +- 13.12% )  (71.55%)
               2,916      LLC-loads:u                      #  396.331 K/sec                    ( +-  7.24% )  (57.37%)
                 469      LLC-load-misses:u                #   18.22% of all LL-cache accesses  ( +- 33.36% )  (46.20%)

            0.007804 +- 0.000243 seconds time elapsed  ( +-  3.12% )
 ```

  ### `Release`

  ```
  $ cargo build -r
    Compiling iso v0.1.0 (/home/lewolfie/Files/Projects/ISO)
    Finished release [optimized] target(s) in 1.27s
  $ perf stat -r 10 -d target/release/iso
    Performance counter stats for 'target/release/iso' (10 runs):

                2.45 msec task-clock:u                     #    0.871 CPUs utilized            ( +-  3.64% )
                   0      context-switches:u               #    0.000 /sec
                   0      cpu-migrations:u                 #    0.000 /sec
                  97      page-faults:u                    #   40.955 K/sec                    ( +-  0.53% )
             991,768      cycles:u                         #    0.419 GHz                      ( +- 24.99% )  (14.30%)
           4,521,401      instructions:u                   #    3.56  insn per cycle           ( +-  5.87% )  (54.53%)
             703,223      branches:u                       #  296.912 M/sec                    ( +-  2.74% )  (87.11%)
              14,119      branch-misses:u                  #    2.14% of all branches          ( +- 31.34% )
             613,371      L1-dcache-loads:u                #  258.975 M/sec                    ( +-  0.01% )
              10,666      L1-dcache-load-misses:u          #    1.74% of all L1-dcache accesses  ( +-  5.87% )  (85.70%)
               2,793      LLC-loads:u                      #    1.179 M/sec                    ( +- 16.24% )  (12.89%)
       <not counted>      LLC-load-misses:u                                             (0.00%)

            0.002808 +- 0.000105 seconds time elapsed  ( +-  3.75% )
  ```
</details>

[1]: https://youtube.com/watch?v=dW1pKnmx_M0
[2]: https://youtube.com/watch?v=cvDyQUpaFf4
[3]: https://youtube.com/watch?v=wY70NCW98Is
[4]: https://www.fastcompany.com/90229646/heres-how-gdpr-is-already-changing-web-design
[5]: https://youtube.com/watch?v=e64_1DmbgN4

## Usage

<sup>Just use our website! The rest is amiss, for now; if you're talking about hosting.</sup>

![Check](badges/check-it-out.svg)

## History

<sup>Still in development!</sup>

We initially developed the website to be a mind-blowing introduction to our company, which used to be *only* an art studio, called ISO Studios. However, we have rethought the website and our company to make more than *just* art, and host our content on top (such as manga, anime, shows, music and games) on a web platform, aside the identity that we embodied on the website.

We are still in really early stages of development, so developers that support our ideal are more than welcome!

## Support Us

Subscribe to our content! We're a commercially-maintained company; donations aren't necessary in an open-source context.

Want to contribute? Do it!

## Contributions

![Developers](https://raw.githubusercontent.com/BraveUX/for-the-badge/master/src/images/badges/built-by-developers.svg)

This project belongs to Fultonic Entertainment, evidently; and thus depends on it. Moreover, the unique development of the website might keep away any new developer to catch up with the code. 

However, it has been made open-source in the spirit of transparency and spring an open-source ecosystem that boosts the project. 

Anyone is welcome to contribute, report issues and raise ideas/suggestions for the website or any other project!

* Want to become a developer for the project? Sure! Just ask!
* Want to create content with us? Sure! Just ask!
* Anything else you want to help us with? ^

## License
    
Code of this repository is under the AGPL 3.0; <sup>[a](LICENSE)</sup> whereas static content (e.g. images, videos or other media) is either *just* copyrighted (i.e. in the case belongs to us or otherwise) or is respectively licensed, such as fonts and icons, which are awknowledged afterward, and are not in the scope of the license.
    
### Badges

Both Askama and AGPL badges are licensed under CC BY 4.0 per FontAwesome's license on Torii Gate icon, <sup>[a][CC] [b][FA]</sup> whereas Check It Out badge is under CC0. <sup>[c][CC0]</sup>

## Acknowledgments

* [Font Awesome](https://github.com/FortAwesome/Font-Awesome/) [(CC BY 4.0)][CC] [<sup>(see LICENSE)</sup>](https://github.com/FortAwesome/Font-Awesome/blob/6.x/LICENSE.txt)
* [Google Fonts](https://github.com/google/fonts) <sup>a</sup> [(OFL & UFL)](https://github.com/google/fonts#license) <sup>b</sup>

[CC]: https://creativecommons.org/licenses/by/4.0/
[CC0]: https://creativecommons.org/publicdomain/zero/1.0/
[FA]: https://github.com/FortAwesome/Font-Awesome/blob/6.x/LICENSE.txt

<div>
    <sup>(a)
        <a href="https://www.theregister.com/2022/01/31/website_fine_google_fonts_gdpr/">
            Self-hosted <img src="https://raw.githubusercontent.com/twitter/twemoji/master/assets/svg/1f921.svg" width=12 />
        </a>
        <sup>
            <a href="https://github.com/twitter/twemoji/blob/master/assets/svg/1f921.svg">
                <sup>(source)</sup>
            <a>
        </sup>
    </sup>
</div>

<div>
    <sup>(b)</sup>
    <a href="https://github.com/google/fonts/tree/main/ufl/ubuntucondensed/LICENCE.txt">
        <sup>Ubuntu Condensed (UFL);</sup>
    </a>
    <a href="https://github.com/google/fonts/blob/main/ofl/unbounded/OFL.txt">
        <sup>other fonts like Unbounded use OFL.</sup>
    </a>
</div>

### README

* [Pake README.md](https://github.com/tw93/Pake/blob/master/README.md)
* [Badges4 README.md](https://github.com/alexandresanlim/Badges4-README.md-Profile)
* [Shields IO](https://github.com/badges/shields) [(CC0 1.0)](https://creativecommons.org/publicdomain/zero/1.0/)
* [For The Badge](https://github.com/BraveUX/for-the-badge/)

___

<div align="center">
    <sup>SVG AGPL logo taken from OpenClipArt
        <a href="https://openclipart.org/detail/89197/agpl%20license%20web%20badge%20(version%202)"><sup>a</sup></a>
        made by Brad Phillips
        <a href="https://openclipart.org/artist/pianoBrad"><sup>b</sup></a>
        under CC0 1.0 (i.e. Public Domain)
        <a href="https://creativecommons.org/publicdomain/zero/1.0/"><sup>c</sup></a>
    </sup>
</div>


<div align="center">
    <sup>SVG Torii Gate icon (from Jinja logo)
        <a href="https://jinja.palletsprojects.com/en/3.1.x/_images/jinja-logo.png"><sup>a</sup></a>
        taken from FontAwesome
        <a href="https://github.com/FortAwesome/Font-Awesome/blob/6.x/svgs/solid/torii-gate.svg"><sup>b</sup></a>
        under CC BY 4.0
        <a href="https://creativecommons.org/licenses/by/4.0/"><sup>c</sup></a>
        <a href="https://github.com/FortAwesome/Font-Awesome/blob/6.x/LICENSE.txt"><sup>(see LICENSE)</sup></a>
    </sup>
</div>

<div align="center">
    <sup>Badges made with the help of Shields IO
        <a href="https://github.com/badges/shields/"><sup>a</sup></a>
        For The Badge's style
        <a href="https://forthebadge.com/"><sup>b</sup></a>
        under CC0 1.0
        <a href="https://creativecommons.org/publicdomain/zero/1.0/"><sup>c</sup></a>
        and For The Badge's generator
        <a href="https://github.com/ekfuhrmann/badge-generator"><sup>d</sup></a>
    </sup>
</div>

<div align="center">
    <a href="https://creativecommons.org/licenses/by/4.0/">
        <img src="https://mirrors.creativecommons.org/presskit/icons/cc.svg" />
        <img src="https://mirrors.creativecommons.org/presskit/icons/by.svg" />
        <p align="center"><sup>Badges under CC BY 4.0 per FontAwesome license</sup></p>
    </a>
</div>

___

<div align="center">
    <img src="https://raw.githubusercontent.com/BraveUX/for-the-badge/master/src/images/badges/built-with-love.svg" />
</div>
