pub const DATA: &str = r#"{
  "addons": [
    {
      "dependencies": {
        "meta_colors": {},
        "meta_languages": {},
        "open_ext": {},
        "settings": {}
      },
      "description": "A metapackage that contains a number of useful plugins to ship with an expanded default version of lite-xl.",
      "id": "meta_addons",
      "mod_version": "3",
      "type": "meta",
      "version": "0.1"
    },
    {
      "dependencies": {
        "abyss": {},
        "betelgeuse": {},
        "c0mfy": {},
        "cold_lime": {},
        "dracula": {},
        "duorand": {},
        "duotone": {},
        "everforest": {},
        "github": {},
        "github-dark-dimmed": {},
        "github_dark": {},
        "gruvbox_dark": {},
        "gruvbox_light": {},
        "jb-fleet": {},
        "jellybeans": {},
        "liqube": {},
        "mariana": {},
        "moe": {},
        "monodark": {},
        "monokai": {},
        "monokai-pro-classic": {},
        "nord": {},
        "onedark": {},
        "only_dark": {},
        "plasma": {},
        "rose-pine": {},
        "rose-pine-dawn": {},
        "rose-pine-moon": {},
        "solarized_dark": {},
        "solarized_light": {},
        "solarobj": {},
        "synthwave": {},
        "tokyo-night": {},
        "vscode-dark": {},
        "winter": {},
        "zenburn": {}
      },
      "description": "A metapackage containing all publically accessible color themes.",
      "id": "meta_colors",
      "mod_version": "3",
      "type": "meta",
      "version": "0.1"
    },
    {
      "dependencies": {
        "language_angelscript": {},
        "language_assembly_riscv": {},
        "language_assembly_x86": {},
        "language_autohotkey_v1": {},
        "language_batch": {},
        "language_bazel": {},
        "language_bib": {},
        "language_blade": {},
        "language_blueprint": {},
        "language_brainfuck": {},
        "language_buzz": {},
        "language_c7": {},
        "language_caddyfile": {},
        "language_carbon": {},
        "language_clojure": {},
        "language_cmake": {},
        "language_containerfile": {},
        "language_crystal": {},
        "language_csharp": {},
        "language_cue": {},
        "language_d": {},
        "language_dart": {},
        "language_diff": {},
        "language_edp": {},
        "language_ejs": {},
        "language_elixir": {},
        "language_elm": {},
        "language_env": {},
        "language_erb": {},
        "language_fe": {},
        "language_fennel": {},
        "language_fortran": {},
        "language_fstab": {},
        "language_gabc": {},
        "language_gdscript": {},
        "language_glsl": {},
        "language_gmi": {},
        "language_go": {},
        "language_graphql": {},
        "language_gravity": {},
        "language_groovy": {},
        "language_hare": {},
        "language_haxe": {},
        "language_hlsl": {},
        "language_hs": {},
        "language_htaccess": {},
        "language_ignore": {},
        "language_ini": {},
        "language_java": {},
        "language_jiyu": {},
        "language_json": {},
        "language_jsx": {},
        "language_julia": {},
        "language_kotlin": {},
        "language_lilypond": {},
        "language_liquid": {},
        "language_lobster": {},
        "language_lox": {},
        "language_make": {},
        "language_marte": {},
        "language_meson": {},
        "language_miniscript": {},
        "language_moon": {},
        "language_nelua": {},
        "language_nginx": {},
        "language_nim": {},
        "language_nix": {},
        "language_objc": {},
        "language_odin": {},
        "language_openscad": {},
        "language_perl": {},
        "language_php": {},
        "language_pico8": {},
        "language_pkgbuild": {},
        "language_po": {},
        "language_powershell": {},
        "language_psql": {},
        "language_r": {},
        "language_rescript": {},
        "language_rivet": {},
        "language_ruby": {},
        "language_rust": {},
        "language_sass": {},
        "language_scala": {},
        "language_sh": {},
        "language_ssh_config": {},
        "language_swift": {},
        "language_tal": {},
        "language_tcl": {},
        "language_teal": {},
        "language_tex": {},
        "language_toml": {},
        "language_ts": {},
        "language_tsx": {},
        "language_typst": {},
        "language_umka": {},
        "language_v": {},
        "language_wren": {},
        "language_yaml": {},
        "language_zig": {}
      },
      "description": "A metapackage containing all publically accessible language syntaxes.",
      "id": "meta_languages",
      "mod_version": "3",
      "type": "meta",
      "version": "0.1.15"
    },
    {
      "description": "Align multiple carets and selections *([clip](https://user-images.githubusercontent.com/2798487/165631951-532f8d24-d596-4dd0-9d21-ff53c71ed32f.mp4))*",
      "id": "align_carets",
      "mod_version": "3",
      "path": "plugins/align_carets.lua",
      "version": "0.1"
    },
    {
      "description": "Automatically inserts closing brackets and quotes. Also allows selected text to be wrapped with brackets or quotes.",
      "id": "autoinsert",
      "mod_version": "3",
      "path": "plugins/autoinsert.lua",
      "version": "0.2"
    },
    {
      "description": "Automatically saves files when they are changed",
      "id": "autosave",
      "mod_version": "3",
      "path": "plugins/autosave.lua",
      "version": "0.2"
    },
    {
      "description": "Automatically saves files that were changed when the main window loses focus by switching to another application",
      "id": "autosaveonfocuslost",
      "mod_version": "3",
      "path": "plugins/autosaveonfocuslost.lua",
      "version": "0.2"
    },
    {
      "description": "Automatically hardwraps lines when typing",
      "id": "autowrap",
      "mod_version": "3",
      "path": "plugins/autowrap.lua",
      "version": "0.1"
    },
    {
      "description": "Theme manager with base16 color schemes for Lite XL.",
      "id": "base16",
      "mod_version": "3",
      "remote": "https://github.com/SmileYzn/base16.git:55b1c3fda3afe7dc2dd894f258389c64b9441da9",
      "version": "0.4"
    },
    {
      "description": "Shows the current time and date in a view with large text *([screenshot](https://user-images.githubusercontent.com/3920290/82752891-3318df00-9db9-11ea-803f-261d80d5cf53.png))*",
      "id": "bigclock",
      "mod_version": "3",
      "path": "plugins/bigclock.lua",
      "version": "0.1"
    },
    {
      "description": "Integrates the [black](https://github.com/psf/black) Python formatter with lite",
      "id": "black",
      "mod_version": "3",
      "remote": "https://git.sr.ht/~tmpod/black-lite:2a1ab1b703f954edb39efb73e72b44c0d18b30a2",
      "version": "0.1"
    },
    {
      "description": "Underlines matching pair for bracket under the caret *([screenshot](https://user-images.githubusercontent.com/3920290/80132745-0c863f00-8594-11ea-8875-c455c6fd7eae.png))*",
      "id": "bracketmatch",
      "mod_version": "3",
      "path": "plugins/bracketmatch.lua",
      "version": "0.2"
    },
    {
      "description": "Provides a build system, messages window, and easily clickable errors. Supports an internal build system, and `make`. *([screenshot](https://raw.githubusercontent.com/lite-xl/lite-xl-ide/main/screenshots/build.png))*",
      "id": "build",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:95639db0f96d8d560d48fa525496329dcbadbbde",
      "version": "0.1"
    },
    {
      "description": "Centers document's content on the screen and adds zen mode support *([screenshot](https://user-images.githubusercontent.com/3920290/82127896-bf6e4500-97ae-11ea-97fc-ba9a552bc9a4.png))*",
      "id": "centerdoc",
      "mod_version": "3",
      "path": "plugins/centerdoc.lua",
      "version": "0.1"
    },
    {
      "description": "Hides the treeview at start if no arguments passed to the executable are directories. Compatible with lite.",
      "id": "cleanstart",
      "mod_version": "3",
      "path": "plugins/cleanstart.lua",
      "version": "0.1"
    },
    {
      "description": "Offers improvements such as highlighted comments and autocomplete for brackets, quotes and more. ([demo](https://s12.gifyu.com/images/Sckre.gif))",
      "id": "codeplus",
      "mod_version": 3,
      "remote": "https://github.com/chqs-git/code-plus.git:b336999b707709df2ddcb865f664623c5beceabb",
      "version": "1.0"
    },
    {
      "dependencies": {
        "widget": {}
      },
      "description": "Color picker dialog that supports html and rgb notations.",
      "id": "colorpicker",
      "mod_version": "3",
      "path": "plugins/colorpicker.lua",
      "version": "0.1"
    },
    {
      "description": "Underlays color values (eg. `#ff00ff` or `rgb(255, 0, 255)`) with their resultant color. *([screenshot](https://user-images.githubusercontent.com/3920290/80743752-731bd780-8b15-11ea-97d3-847db927c5dc.png))*",
      "id": "colorpreview",
      "mod_version": "3",
      "path": "plugins/colorpreview.lua",
      "version": "0.2"
    },
    {
      "description": "A console for running external commands and capturing their output *([gif](https://user-images.githubusercontent.com/3920290/81343656-49325a00-90ad-11ea-8647-ff39d8f1d730.gif))*",
      "id": "console",
      "mod_version": "3",
      "remote": "https://github.com/franko/console:08c3f5e1483627c3ae688ce5a99008e823357f2c",
      "version": "0.1"
    },
    {
      "description": "Copy file location to clipboard",
      "id": "copyfilelocation",
      "mod_version": "3",
      "path": "plugins/copyfilelocation.lua",
      "version": "0.1"
    },
    {
      "description": "Customize the caret in the editor",
      "id": "custom_caret",
      "mod_version": "3",
      "path": "plugins/custom_caret.lua",
      "version": "0.3"
    },
    {
      "description": "Insert date-, time- and date-time-stamps",
      "id": "datetimestamps",
      "mod_version": "3",
      "path": "plugins/datetimestamps.lua",
      "version": "0.1"
    },
    {
      "description": "Icons font for the devicons plugin.",
      "id": "font_devicons",
      "mod_version": "3",
      "remote": "https://github.com/PerilousBooklet/lite-xl-devicons:e78f2374a1f4de5d0f324af46a07c2f9ffa7ccef",
      "type": "font",
      "version": "1.0.0"
    },
    {
      "description": "PerilousBooklet's treeview icons for software developers.",
      "id": "devicons",
      "mod_version": "3",
      "remote": "https://github.com/PerilousBooklet/lite-xl-devicons:e78f2374a1f4de5d0f324af46a07c2f9ffa7ccef",
      "version": "1.0.0"
    },
    {
      "description": "Add end tags in variety of languages. Similar to [Vim Endwise](https://github.com/tpope/vim-endwise)",
      "id": "endwise",
      "mod_version": "3",
      "remote": "https://github.com/LolsonX/endwise-lite-xl.git:d9ced6089ab7f54bad197c30ba581055270f2dad",
      "version": "0.1"
    },
    {
      "description": "Automatically displays a font with increasing sizes in a new view.",
      "id": "fontpreview",
      "mod_version": "3",
      "path": "plugins/fontpreview.lua",
      "version": "0.1"
    },
    {
      "description": "Debug Lite-XL's Lua VM interactively, if you're running it from a terminal. Warning: Will significantly slow down Lite-XL if installed.",
      "id": "lite-debugger",
      "mod_version": "3",
      "path": "plugins/lite-debugger.lua",
      "version": "0.1"
    },
    {
      "description": "Provides a debugger integration, with pluggable backends. Currently supports only gdb. *([screenshot](https://raw.githubusercontent.com/lite-xl/lite-xl-ide/main/screenshots/debugger.png))*",
      "id": "debugger",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:95639db0f96d8d560d48fa525496329dcbadbbde",
      "version": "0.1"
    },
    {
      "description": "Adds the current workspace and file to your Discord rich presence. Linux-only.",
      "id": "discord-presence",
      "mod_version": "3",
      "name": "Discord RPC",
      "remote": "https://github.com/vincens2005/lite-xl-discord2:71648e55ad812dc1029a3df70c17656ec7cb63bf",
      "version": "0.2"
    },
    {
      "description": "Provides basic drag and drop of selected text (in same document)",
      "id": "dragdropselected",
      "mod_version": "3",
      "path": "plugins/dragdropselected.lua",
      "version": "0.2"
    },
    {
      "description": "Adds a popup that displays the curve of Penner-styled easing functions.",
      "id": "easingpreview",
      "mod_version": "3",
      "name": "Easing Previewer",
      "remote": "https://github.com/ThaCuber/lite-xl-easingpreview:ea349d12a9570264408dc1567af660b8d4568d3d",
      "version": "1.0"
    },
    {
      "description": "[EditorConfig](https://editorconfig.org/) implementation for Lite XL",
      "id": "editorconfig",
      "mod_version": "3",
      "path": "plugins/editorconfig",
      "version": "0.1"
    },
    {
      "description": "Add support for detecting file and string encodings as converting between them.",
      "id": "encoding",
      "mod_version": "3",
      "remote": "https://github.com/jgmdev/lite-xl-encoding:16e2477e916f52e18f6d63f5ac61ace58b0c5e45",
      "type": "library",
      "version": "1.1"
    },
    {
      "dependencies": {
        "encoding": {}
      },
      "description": "Properly read files that are not encoded in UTF-8 or ASCII by auto-detecting their encoding and allows saving on different text encodings.",
      "id": "encodings",
      "mod_version": "3",
      "remote": "https://github.com/jgmdev/lite-xl-encoding:16e2477e916f52e18f6d63f5ac61ace58b0c5e45",
      "version": "1.0"
    },
    {
      "checksum": "80c07430455e665b5f5009667fe76458603af505ce411a38b38e82e7604d95f0",
      "description": "Make sure the file ends with one blank line.",
      "id": "eofnewline",
      "mod_version": "3",
      "url": "https://github.com/bokunodev/lite_modules/blob/master/plugins/eofnewline-xl.lua?raw=1",
      "version": "0.1"
    },
    {
      "description": "Preview tabs. Opening a doc will replace the contents of the preview tab. Marks tabs as non-preview on any change or tab double clicking.",
      "id": "ephemeral_tabs",
      "mod_version": "3",
      "path": "plugins/ephemeral_tabs.lua",
      "version": "0.1"
    },
    {
      "description": "Graphs y=x equations.",
      "id": "equationgrapher",
      "mod_version": "3",
      "remote": "https://github.com/thacuber2a03/equationgrapher:7b17d8de47ab5c40019c1910a303340b5176e105",
      "version": "0.2"
    },
    {
      "description": "Replaces selected Lua code with its evaluated result",
      "id": "eval",
      "mod_version": "3",
      "path": "plugins/eval.lua",
      "version": "0.1"
    },
    {
      "description": "Adds Treesitter syntax highlighting support",
      "id": "evergreen",
      "mod_version": "3",
      "remote": "https://github.com/TorchedSammy/Evergreen.lxl:e5e6ade897a6316fbe6fc610520d195e27737912",
      "version": "0.2"
    },
    {
      "description": "Runs selected text through shell command and replaces with result",
      "id": "exec",
      "mod_version": "3",
      "path": "plugins/exec.lua",
      "version": "0.1"
    },
    {
      "description": "When a selection crosses multiple lines, it is drawn to the end of the screen *([screenshot](https://user-images.githubusercontent.com/2798487/140995616-89a20b55-5917-4df8-8a7c-d7c53732fa8b.png))*",
      "id": "extend_selection_line",
      "mod_version": "3",
      "path": "plugins/extend_selection_line.lua",
      "version": "0.1"
    },
    {
      "description": "Allows to open an external console in current project directory",
      "id": "exterm",
      "mod_version": "3",
      "remote": "https://github.com/ShadiestGoat/lite-xl-exterm:aca8827fc1af831890cffd3dd122debac72429c6",
      "version": "0.1"
    },
    {
      "description": "Adds support for fallback fonts *([gif](https://raw.githubusercontent.com/takase1121/lite-fallback-fonts/master/assets/Iw18fI57J0.gif))*",
      "id": "fallbackfonts",
      "mod_version": "3",
      "remote": "https://github.com/takase1121/lite-fallback-fonts:281cafc014f7931f041046f76496797695678bb4",
      "version": "0.1"
    },
    {
      "dependencies": {
        "thread": {}
      },
      "description": "Threaded project find files.",
      "id": "findfileimproved",
      "mod_version": "3",
      "name": "Multithreaded Find File",
      "remote": "https://github.com/jgmdev/lite-xl-threads:9299a9a6b778cb34b12f0286b9162779920a9197",
      "version": "1.0"
    },
    {
      "description": "[Nonicons](https://github.com/yamatsum/nonicons/) font with mapping",
      "files": [
        {
          "checksum": "da0a065856c44ea3fd3b9fba2fff5e25a32992dc3a3c5df6e9bfca2cd106a8cc",
          "url": "https://github.com/yamatsum/nonicons/raw/8454b3b6c3ceeee18b386b7882c5a071dcf0f3af/dist/nonicons.ttf"
        }
      ],
      "id": "font_nonicons",
      "name": "Nonicons font",
      "path": "plugins/font_nonicons.lua",
      "type": "library",
      "version": "20230530"
    },
    {
      "description": "[Nerd Font Symbols](https://github.com/ryanoasis/nerd-fonts/) font with mapping",
      "files": [
        {
          "checksum": "f4f40e003e9d00000f1218a606127fab82b1faf0df2d9095a06190f4f2c32e36",
          "url": "https://github.com/ryanoasis/nerd-fonts/raw/cf3561b1a51f70d5398eb49d603679b0f0c55d74/patched-fonts/NerdFontsSymbolsOnly/SymbolsNerdFontMono-Regular.ttf"
        }
      ],
      "id": "font_symbols_nerdfont_mono_regular",
      "name": "Nerd Font Symbols Mono Regular",
      "path": "plugins/font_symbols_nerdfont_mono_regular.lua",
      "type": "library",
      "version": "3.1.1"
    },
    {
      "description": "Allows users to load fonts with [fontconfig](https://www.freedesktop.org/software/fontconfig/fontconfig-user.html).",
      "id": "fontconfig",
      "mod_version": "3",
      "path": "plugins/fontconfig.lua",
      "version": "0.1"
    },
    {
      "description": "Change the syntax used for a file.",
      "id": "force_syntax",
      "mod_version": "3",
      "path": "plugins/force_syntax.lua",
      "version": "0.1"
    },
    {
      "description": "formatters for various languages",
      "id": "formatter",
      "mod_version": "3",
      "remote": "https://github.com/vincens2005/lite-formatters:7e017080a967c01d437e484247a90c1ff52e8ef8",
      "version": "0.1"
    },
    {
      "description": "Opens a preview of the current markdown file in a browser window *([screenshot](https://user-images.githubusercontent.com/3920290/82754898-f7394600-9dc7-11ea-8278-2305363ed372.png))* Note: the page content is generated by sending the markdown file to Github's markdown rendering [API](https://docs.github.com/en/rest/markdown?apiVersion=2022-11-28). Requires a [GitHub token](https://docs.github.com/en/rest/markdown/markdown?apiVersion=2022-11-28#render-a-markdown-document-in-raw-mode) being provided",
      "id": "ghmarkdown",
      "mod_version": "3",
      "path": "plugins/ghmarkdown.lua",
      "version": "0.2"
    },
    {
      "description": "Shows \"git blame\" information of a line *([screenshot](https://raw.githubusercontent.com/juliardi/lite-xl-gitblame/main/screenshot_1.png))*",
      "id": "gitblame",
      "mod_version": "3",
      "remote": "https://github.com/juliardi/lite-xl-gitblame:0395fed18d3a779bc1eae26130f00a2eb23638ad",
      "version": "0.2"
    },
    {
      "description": "highlight changed lines from git *([screenshot](https://raw.githubusercontent.com/vincens2005/lite-xl-gitdiff-highlight/master/screenshot.png))*",
      "id": "gitdiff_highlight",
      "mod_version": "3",
      "remote": "https://github.com/vincens2005/lite-xl-gitdiff-highlight:f0e02b6a7299acbeb4a5f137b26830a6cca96cc8",
      "version": "0.2"
    },
    {
      "description": "Open project files that are in a git commit (default=HEAD)",
      "id": "gitopen",
      "mod_version": "3",
      "path": "plugins/gitopen.lua",
      "version": "0.1"
    },
    {
      "description": "Displays git branch and insert/delete count in status bar *([screenshot](https://user-images.githubusercontent.com/3920290/81107223-bcea3080-8f0e-11ea-8fc7-d03173f42e33.png))*",
      "id": "gitstatus",
      "mod_version": "3",
      "path": "plugins/gitstatus.lua",
      "version": "0.2"
    },
    {
      "description": "Auto-formats the current go file, adds the missing imports and the missing return cases",
      "id": "gofmt",
      "mod_version": "3",
      "path": "plugins/gofmt.lua",
      "version": "0.1"
    },
    {
      "description": "Graphical filepicker using zenity or kdialog.",
      "id": "gui_filepicker",
      "mod_version": "3",
      "path": "plugins/gui_filepicker.lua",
      "version": "1.0"
    },
    {
      "description": "Dark (or even Mica!) title bar for Lite XL",
      "id": "immersive-title",
      "mod_version": "3",
      "remote": "https://github.com/takase1121/lite-xl-immersive-title:cf3a8029ac7154ea53ac819a95d44a6ff102f051",
      "version": "0.1"
    },
    {
      "description": "Convert between tabs and spaces indentation",
      "id": "indent_convert",
      "mod_version": "3",
      "path": "plugins/indent_convert.lua",
      "version": "0.1"
    },
    {
      "description": "Adds indent guides *([screenshot](https://user-images.githubusercontent.com/3920290/79640716-f9860000-818a-11ea-9c3b-26d10dd0e0c0.png))*",
      "id": "indentguide",
      "mod_version": "3",
      "path": "plugins/indentguide.lua",
      "version": "0.2"
    },
    {
      "description": "Adds inter-process communication support, single-instance mode and tab drag and drop between instances.",
      "id": "ipc",
      "mod_version": "3",
      "path": "plugins/ipc.lua",
      "version": "0.3"
    },
    {
      "description": "Production and Early-Access OpenJDK Builds, from Oracle.",
      "id": "jdk",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:89efc5642cb9abcd9023c8e5ef3dba85b399884e",
      "type": "library",
      "version": "21.0.2"
    },
    {
      "description": "Exports the keymap to a JSON file.",
      "id": "keymap_export",
      "mod_version": "3",
      "path": "plugins/keymap_export.lua",
      "version": "0.1"
    },
    {
      "dependencies": {
        "console": {}
      },
      "description": "Adds [Kinc](https://github.com/Kode/Kinc) Project generation with basic build commands(depends on [`console`](https://github.com/franko/console))",
      "id": "kinc-projects",
      "mod_version": "3",
      "name": "Kinc Projects",
      "remote": "https://github.com/Kode-Community/kinc_plugin:309fe4193a09cf739ed0a058b1a6966a463a1dbd",
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Angelscript](https://www.angelcode.com/angelscript/) programming language",
      "id": "language_angelscript",
      "mod_version": "3",
      "path": "plugins/language_angelscript.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for Intel x86 assembly",
      "id": "language_assembly_x86",
      "mod_version": "3",
      "path": "plugins/language_assembly_x86.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for RISC-V assembly",
      "id": "language_assembly_riscv",
      "mod_version": "3",
      "path": "plugins/language_assembly_riscv.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [AutoHotkey](https://www.autohotkey.com)(v1) programming language",
      "id": "language_autohotkey_v1",
      "mod_version": "3",
      "path": "plugins/language_autohotkey_v1.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for Windows [Batch Files](https://en.wikipedia.org/wiki/Batch_file)",
      "id": "language_batch",
      "mod_version": "3",
      "path": "plugins/language_batch.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [Bazel](https://bazel.build/) build tool files.",
      "id": "language_bazel",
      "mod_version": "3",
      "path": "plugins/language_bazel.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [BibTex](https://en.wikipedia.org/wiki/BibTeX) files",
      "id": "language_bib",
      "mod_version": "3",
      "path": "plugins/language_bib.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [Blade](https://github.com/blade-lang/blade/) files",
      "id": "language_blade",
      "mod_version": "3",
      "path": "plugins/language_blade.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Blueprint](https://jwestman.pages.gitlab.gnome.org/blueprint-compiler/) markup language",
      "id": "language_blueprint",
      "mod_version": "3",
      "path": "plugins/language_blueprint.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) programming language",
      "id": "language_brainfuck",
      "mod_version": "3",
      "path": "plugins/language_brainfuck.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "A syntax highlighter for the [Buzz programming language](https://buzz-lang.dev/) in lite-xl.",
      "id": "language_buzz",
      "mod_version": "3",
      "path": "plugins/language_buzz.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the modifications to [fe](https://github.com/rxi/fe/) used in [cel7](https://rxi.itch.io/cel7)",
      "id": "language_c7",
      "mod_version": "3",
      "path": "plugins/language_c7.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the Caddyfile used on the [Caddy](https://caddyserver.com/) web server",
      "id": "language_caddyfile",
      "mod_version": "3",
      "path": "plugins/language_caddyfile.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Carbon programming language](https://github.com/carbon-language/carbon-lang).",
      "id": "language_carbon",
      "mod_version": "3",
      "path": "plugins/language_carbon.lua",
      "tags": [
        "language"
      ],
      "version": "0.3"
    },
    {
      "description": "Syntax for the [Clojure](https://clojure.org/) programming language",
      "id": "language_clojure",
      "mod_version": "3",
      "path": "plugins/language_clojure.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the CMake build system language",
      "id": "language_cmake",
      "mod_version": "3",
      "path": "plugins/language_cmake.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for [Containerfile](https://github.com/containers/common/blob/main/docs/Containerfile.5.md)/[Dockerfile](https://docs.docker.com/engine/reference/builder/)",
      "id": "language_containerfile",
      "mod_version": "3",
      "remote": "https://github.com/FilBot3/lite-xl-language-containerfile:ae4eddc3f3fa1a0db56344b3b6db0ec0f2283880",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [Crystal](https://crystal-lang.org) programming language",
      "id": "language_crystal",
      "mod_version": "3",
      "remote": "https://github.com/Tamnac/lite-plugin-crystal:1913a6dd23a7640b507943230291a15aecc3235d",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [C#](http://csharp.net) programming language",
      "id": "language_csharp",
      "mod_version": "3",
      "path": "plugins/language_csharp.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [CUE](http://cuelang.org) definition and validation programming language",
      "id": "language_cue",
      "mod_version": "3",
      "path": "plugins/language_cue.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [D](https://dlang.org/) programming language",
      "id": "language_d",
      "mod_version": "3",
      "path": "plugins/language_d.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [Dart](https://dart.dev/) programming languiage",
      "id": "language_dart",
      "mod_version": "3",
      "path": "plugins/language_dart.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for diff and patch files",
      "id": "language_diff",
      "mod_version": "3",
      "path": "plugins/language_diff.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Djot](https://djot.net/) markup language",
      "id": "language_djot",
      "mod_version": "3",
      "remote": "https://github.com/Tamnac/lite-xl-djot:e92ad996aba56c3363ef83a2dd8d1026be56c730",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [FreeFEM++](http://freefem.org) programming language",
      "id": "language_edp",
      "mod_version": "3",
      "path": "plugins/language_edp.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [EJS](https://ejs.co/) javascript template engine",
      "id": "language_ejs",
      "mod_version": "3",
      "path": "plugins/language_ejs.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Elixir](https://elixir-lang.org) programming language",
      "id": "language_elixir",
      "mod_version": "3",
      "path": "plugins/language_elixir.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [Elm](https://elm-lang.org) programming language",
      "id": "language_elm",
      "mod_version": "3",
      "path": "plugins/language_elm.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [env](https://hexdocs.pm/dotenvy/dotenv-file-format.html) (dotenv) files",
      "id": "language_env",
      "mod_version": "3",
      "remote": "https://github.com/anthonyaxenov/lite-xl-env-syntax:89a4274af6a786963930d7a8fb542dfe61daa801",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [ERB](https://github.com/ruby/erb) programming language. Also known as eRuby or Embedded Ruby.",
      "id": "language_erb",
      "mod_version": "3",
      "path": "plugins/language_erb.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [fe](https://github.com/rxi/fe) programming language",
      "id": "language_fe",
      "mod_version": "3",
      "path": "plugins/language_fe.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [fennel](https://fennel-lang.org) programming language",
      "id": "language_fennel",
      "mod_version": "3",
      "path": "plugins/language_fennel.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [fortran](https://fortran-lang.org/) programming language",
      "id": "language_fortran",
      "mod_version": "3",
      "path": "plugins/language_fortran.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [fstab](https://en.wikipedia.org/wiki/Fstab) config files",
      "id": "language_fstab",
      "mod_version": "3",
      "path": "plugins/language_fstab.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [GABC](https://gregorio-project.github.io/gabc) music typesetting language",
      "id": "language_gabc",
      "mod_version": "3",
      "path": "plugins/language_gabc.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Godot Engine](https://godotengine.org/)'s GDScript scripting language",
      "id": "language_gdscript",
      "mod_version": "3",
      "path": "plugins/language_gdscript.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [GLSL](https://www.khronos.org/registry/OpenGL/specs/gl/) programming language",
      "id": "language_glsl",
      "mod_version": "3",
      "path": "plugins/language_glsl.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Gemtext](https://gemini.circumlunar.space/docs/gemtext.gmi) markup language",
      "id": "language_gmi",
      "mod_version": "3",
      "path": "plugins/language_gmi.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Go](https://golang.org/) programming language",
      "id": "language_go",
      "mod_version": "3",
      "path": "plugins/language_go.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [GraphQL](https://graphql.org/) query language, and server-side runtime for executing queries using a type system.",
      "id": "language_graphql",
      "mod_version": "3",
      "path": "plugins/language_graphql.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [Gravity](https://marcobambini.github.io/gravity/) programming language.",
      "id": "language_gravity",
      "mod_version": "3",
      "path": "plugins/language_gravity.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Groovy](https://en.wikipedia.org/wiki/Apache_Groovy",
      "id": "language_groovy",
      "mod_version": "3",
      "path": "plugins/language_groovy.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [Hare](https://harelang.org) programming language",
      "id": "language_hare",
      "mod_version": "3",
      "path": "plugins/language_hare.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Haxe](https://haxe.org) programming language",
      "id": "language_haxe",
      "mod_version": "3",
      "path": "plugins/language_haxe.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [HLSL](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl) programming language",
      "id": "language_hlsl",
      "mod_version": "3",
      "path": "plugins/language_hlsl.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Haskell](https://www.haskell.org/) programming language",
      "id": "language_hs",
      "mod_version": "3",
      "path": "plugins/language_hs.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for [.htaccess](https://httpd.apache.org/docs/2.4/howto/htaccess.html) files.",
      "id": "language_htaccess",
      "mod_version": "3",
      "path": "/plugins/language_htaccess.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [.gitignore](https://git-scm.com/docs/gitignore), [.dockerignore](https://docs.docker.com/engine/reference/builder/#dockerignore-file) and some other `.*ignore` files",
      "id": "language_ignore",
      "mod_version": "3",
      "remote": "https://github.com/anthonyaxenov/lite-xl-ignore-syntax:3a9a5e0ae03b82358473da5d1c6012944d65ea95",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [ini](https://en.wikipedia.org/wiki/INI_file) files",
      "id": "language_ini",
      "mod_version": "3",
      "path": "plugins/language_ini.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Java](https://en.wikipedia.org/wiki/Java_\\(programming_language\\)) programming language",
      "id": "language_java",
      "mod_version": "3",
      "path": "plugins/language_java.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [jiyu](https://github.com/machinamentum/jiyu) programming language",
      "id": "language_jiyu",
      "mod_version": "3",
      "path": "plugins/language_jiyu.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [JSX](https://reactjs.org/docs/introducing-jsx.html) language for the React framework in JavaScript",
      "id": "language_jsx",
      "mod_version": "3",
      "path": "plugins/language_jsx.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [JSON](https://www.json.org/json-en.html) language",
      "id": "language_json",
      "mod_version": "3",
      "path": "plugins/language_json.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Julia](https://julialang.org/) programming language",
      "id": "language_julia",
      "mod_version": "3",
      "path": "plugins/language_julia.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Kotlin](https://kotlinlang.org/docs/home.html) programming language",
      "id": "language_kotlin",
      "mod_version": "3",
      "path": "plugins/language_kotlin.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "checksum": "08a9f8635b09a98cec9dfca8bb65f24fd7b6585c7e8308773e7ddff9a3e5a60f",
      "description": "Syntax for [Kaitai](http://kaitai.io/) struct files",
      "id": "language_ksy",
      "mod_version": "1",
      "tags": [
        "language"
      ],
      "url": "https://raw.githubusercontent.com/whiteh0le/lite-plugins/main/plugins/language_ksy.lua?raw=1",
      "version": "0.1"
    },
    {
      "description": "Syntax for the [LilyPond](https://lilypond.org/) music typesetting language",
      "id": "language_lilypond",
      "mod_version": "3",
      "path": "plugins/language_lilypond.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [Liquid](https://shopify.github.io/liquid/) templating language",
      "id": "language_liquid",
      "mod_version": "3",
      "path": "plugins/language_liquid.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [Lobster](https://strlen.com/lobster/) programming language",
      "id": "language_lobster",
      "mod_version": "3",
      "path": "plugins/language_lobster.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Lox](http://craftinginterpreters.com/the-lox-language.html) programming language, featured in the book '[Crafting Interpreters](http://craftinginterpreters.com/)'.",
      "id": "language_lox",
      "mod_version": "3",
      "path": "plugins/language_lox.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the Make build system language",
      "id": "language_make",
      "mod_version": "3",
      "path": "plugins/language_make.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [MARTe2](https://vcis.f4e.europa.eu/marte2-docs/master/html/index.html) configuration language",
      "id": "language_marte",
      "mod_version": "3",
      "path": "plugins/language_marte.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Meson](https://mesonbuild.com) build system language",
      "id": "language_meson",
      "mod_version": "3",
      "path": "plugins/language_meson.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [MiniScript](https://miniscript.org) programming language",
      "id": "language_miniscript",
      "mod_version": "3",
      "path": "plugins/language_miniscript.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [MoonScript](https://moonscript.org) scripting language",
      "id": "language_moon",
      "mod_version": "3",
      "path": "plugins/language_moon.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for [Nelua](http://nelua.io/) programming",
      "id": "language_nelua",
      "mod_version": "3",
      "path": "plugins/language_nelua.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [Nginx](https://www.nginx.com/) config files",
      "id": "language_nginx",
      "mod_version": "3",
      "path": "plugins/language_nginx.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Nim](https://nim-lang.org) programming language",
      "id": "language_nim",
      "mod_version": "3",
      "path": "plugins/language_nim.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Nix](https://nixos.wiki/wiki/Overview_of_the_Nix_Language) expression language",
      "id": "language_nix",
      "mod_version": "3",
      "path": "plugins/language_nix.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Objective C](https://en.wikipedia.org/wiki/Objective-C) programming language",
      "id": "language_objc",
      "mod_version": "3",
      "path": "plugins/language_objc.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Odin](https://github.com/odin-lang/Odin) programming language",
      "id": "language_odin",
      "mod_version": "3",
      "path": "plugins/language_odin.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [OpenSCAD](https://openscad.org/) programming language",
      "id": "language_openscad",
      "mod_version": "3",
      "path": "plugins/language_openscad.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Perl](https://perl.org) programming language",
      "id": "language_perl",
      "mod_version": "3",
      "path": "plugins/language_perl.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [PHP](https://php.net) programming language",
      "id": "language_php",
      "mod_version": "3",
      "path": "plugins/language_php.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [Pico-8](https://www.lexaloffle.com/pico-8.php) cartridge files",
      "id": "language_pico8",
      "mod_version": "3",
      "path": "plugins/language_pico8.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [PKGBUILD](https://wiki.archlinux.org/title/PKGBUILD) package description files",
      "id": "language_pkgbuild",
      "mod_version": "3",
      "path": "plugins/language_pkgbuild.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [PO](https://www.gnu.org/software/gettext/manual/html_node/PO-Files.html) translation files",
      "id": "language_po",
      "mod_version": "3",
      "path": "plugins/language_po.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [Pony](https://www.ponylang.io/) programming language",
      "id": "language_pony",
      "mod_version": "2",
      "remote": "https://github.com/MrAnyx/lite-plugin-pony:34d9ec673eaa6c409bcef0febaa0a36cc3acdf4e",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [PowerShell](https://docs.microsoft.com/en-us/powershell) scripting language",
      "id": "language_powershell",
      "mod_version": "3",
      "path": "plugins/language_powershell.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the postgresql database access language",
      "id": "language_psql",
      "mod_version": "3",
      "path": "plugins/language_psql.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [R](https://www.r-project.org/) scripting language",
      "id": "language_r",
      "mod_version": "3",
      "name": "language_R",
      "path": "plugins/language_R.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [ReScript](https://rescript-lang.org/) programming language",
      "id": "language_rescript",
      "mod_version": "3",
      "path": "plugins/language_rescript.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Rivet](https://github.com/rivet-lang/rivet) programming language",
      "id": "language_rivet",
      "mod_version": "3",
      "path": "plugins/language_rivet.lua",
      "tags": [
        "language"
      ],
      "version": "0.4.5"
    },
    {
      "description": "Syntax for the [Ruby](https://www.ruby-lang.org/) programming language",
      "id": "language_ruby",
      "mod_version": "3",
      "path": "plugins/language_ruby.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Rust](https://rust-lang.org/) programming language",
      "id": "language_rust",
      "mod_version": "3",
      "path": "plugins/language_rust.lua",
      "tags": [
        "language"
      ],
      "version": "0.3"
    },
    {
      "description": "Syntax for the [Sass](https://sass-lang.com/) CSS preprocessor",
      "id": "language_sass",
      "mod_version": "3",
      "path": "plugins/language_sass.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Scala](https://scala-lang.org/) programming language",
      "id": "language_scala",
      "mod_version": "3",
      "path": "plugins/language_scala.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Swift](https://developer.apple.com/swift/) programming language",
      "id": "language_swift",
      "mod_version": "3",
      "path": "plugins/language_swift.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for shell scripting language",
      "id": "language_sh",
      "mod_version": "3",
      "path": "plugins/language_sh.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for ssh & sshd config files",
      "id": "language_ssh_config",
      "mod_version": "3",
      "path": "plugins/language_ssh_config.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Uxntal](https://wiki.xxiivv.com/site/uxntal) assembly language",
      "id": "language_tal",
      "mod_version": "3",
      "path": "plugins/language_tal.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Umka](https://github.com/vtereshkov/umka-lang) programming language.",
      "id": "language_umka",
      "mod_version": "3",
      "path": "plugins/language_umka.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Tcl](https://www.tcl.tk/) programming language",
      "id": "language_tcl",
      "mod_version": "3",
      "path": "plugins/language_tcl.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Teal](https://github.com/teal-language/tl) programming language, a typed dialect of Lua.",
      "id": "language_teal",
      "mod_version": "3",
      "path": "plugins/language_teal.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [LaTeX](https://www.latex-project.org/) typesetting language",
      "id": "language_tex",
      "mod_version": "3",
      "path": "plugins/language_tex.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [TOML](https://toml.io/en/) configuration language",
      "id": "language_toml",
      "mod_version": "3",
      "path": "plugins/language_toml.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [TypeScript](https://www.typescriptlang.org/) programming language, a typed dialect of JavaScript.",
      "id": "language_ts",
      "mod_version": "3",
      "path": "plugins/language_ts.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [TSX](https://www.typescriptlang.org/docs/handbook/jsx.html) language",
      "id": "language_tsx",
      "mod_version": "3",
      "path": "plugins/language_tsx.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for [Typst](https://typst.app/) markup language",
      "id": "language_typst",
      "mod_version": "3",
      "path": "plugins/language_typst.lua",
      "tags": [
        "language"
      ],
      "version": "0.2"
    },
    {
      "description": "Syntax for the [V](https://vlang.io/) programming language",
      "id": "language_v",
      "mod_version": "3",
      "path": "plugins/language_v.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Vale](https://vale.dev) programming language",
      "id": "language_vale",
      "mod_version": "3",
      "remote": "https://github.com/programutox/lite-plugin-vale:faa75f67b093978ceebc31bb7db8aa297f3c3e52",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Wren](http://wren.io/) programming language",
      "id": "language_wren",
      "mod_version": "3",
      "path": "plugins/language_wren.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for [YAML](https://yaml.org/) serialization language",
      "id": "language_yaml",
      "mod_version": "3",
      "path": "plugins/language_yaml.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Syntax for the [Zig](https://ziglang.org/) programming language",
      "id": "language_zig",
      "mod_version": "3",
      "path": "plugins/language_zig.lua",
      "tags": [
        "language"
      ],
      "version": "0.1"
    },
    {
      "description": "Automatically inserts indentation and closing bracket/text after newline",
      "id": "lfautoinsert",
      "mod_version": "3",
      "path": "plugins/lfautoinsert.lua",
      "version": "0.3"
    },
    {
      "description": "The ability to change the display of the line number *([screenshot](https://user-images.githubusercontent.com/5556081/129493788-6a4cbd7a-9074-4133-bab7-110ed55f18f7.png))*",
      "id": "linenumbers",
      "mod_version": "3",
      "path": "plugins/linenumbers.lua",
      "version": "0.2"
    },
    {
      "description": "Advanced linter with ErrorLens-like error reporting. Compatible with linters made for `linter` *([screenshot](https://raw.githubusercontent.com/liquid600pgm/lintplus/master/screenshots/1.png))*",
      "id": "lintplus",
      "mod_version": "3",
      "name": "lint+",
      "remote": "https://github.com/liquid600pgm/lintplus:771b1fe6cddb7897cd034ed5ee96201d6a2831c2",
      "replaces": [
        "linter"
      ],
      "version": "0.2"
    },
    {
      "description": "Discord rich presence for Lite XL (display file editing in Discord)",
      "id": "litepresence",
      "mod_version": "3",
      "remote": "https://github.com/TorchedSammy/Litepresence:1985e8a59feaaea5085a5add2521293a11f6d376",
      "version": "0.1"
    },
    {
      "description": "Generates Lorem Ipsum placeholder dummy text",
      "id": "lorem",
      "mod_version": "3",
      "remote": "https://github.com/sheetcoder/lorem:b2da386519850d6f91ef67097e50141b3b11a90e",
      "version": "0.1"
    },
    {
      "description": "Provides code completion (also known as IntelliSense) using the Language Server Protocol",
      "id": "lsp",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp:b1b77d143b2c6e3cd953b3573ea1a96dd61ff3bb",
      "version": "0.7"
    },
    {
      "description": "Completion menu kind/type icons for Lite XL LSP",
      "id": "lspkind",
      "mod_version": "3",
      "remote": "https://github.com/TorchedSammy/lite-xl-lspkind:272ebd0010cd3e205cfb486b1bae889080fec437",
      "version": "0.1"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for C/C++ with clangd.",
      "id": "lsp_c",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:89efc5642cb9abcd9023c8e5ef3dba85b399884e",
      "version": "17.0.3"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for clojure with clojure-lsp.",
      "id": "lsp_clojure",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:e6460f2711c8a11495be2292e07ec894457b8c16",
      "version": "2024.03.31"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for Emmet with emmet-language-server.",
      "id": "lsp_emmet",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:89efc5642cb9abcd9023c8e5ef3dba85b399884e",
      "version": "2.5.0"
    },
    {
      "description": "LSP support for JSON via vscode-json-languageserver with [additional patches](https://github.com/lite-xl/lite-xl-lsp-servers/patches/vscode-json-languageserver) for increased functionality.",
      "id": "lsp_json",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:69a24616439ea9078df9a7474a4b0cb82f3cf65c",
      "version": "1.87.2.0.1"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for Lua with lua-language-server (sumneko).",
      "id": "lsp_lua",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:89efc5642cb9abcd9023c8e5ef3dba85b399884e",
      "version": "3.7.4"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for Java with jdtls.",
      "id": "lsp_java",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:89efc5642cb9abcd9023c8e5ef3dba85b399884e",
      "version": "1.33.0"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for Python with Pyright.",
      "id": "lsp_python",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers.git:71275f7e47a30ac0cbeb17706cea29a29c8267a7",
      "version": "1.1.356"
    },
    {
      "description": "Automatic configuration/binary download for LSP linting for Javascript with quick-lint-js.",
      "id": "lsp_quicklintjs",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:89efc5642cb9abcd9023c8e5ef3dba85b399884e",
      "version": "3.2.0"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for Rust with rust-analyzer.",
      "id": "lsp_rust",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers.git:69a24616439ea9078df9a7474a4b0cb82f3cf65c",
      "version": "20240325"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for TeX with texlab.",
      "id": "lsp_tex",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers.git:71275f7e47a30ac0cbeb17706cea29a29c8267a7",
      "version": "5.14.0"
    },
    {
      "description": "Automatic configuration/binary download for LSP completion for Typescript and Javascript with typescript-language-server.",
      "id": "lsp_typescript",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:89efc5642cb9abcd9023c8e5ef3dba85b399884e",
      "version": "4.3.3"
    },
    {
      "description": "Adds LSP snippets support to the snippets plugin",
      "id": "lsp_snippets",
      "mod_version": "3",
      "remote": "https://github.com/vqns/lite-xl-snippets:1f0bba02bb3af6df9cb2ce72526851b427caf143",
      "version": "1.1"
    },
    {
      "description": "LSP support for YAML via yaml-language-server.",
      "id": "lsp_yaml",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:69a24616439ea9078df9a7474a4b0cb82f3cf65c",
      "version": "1.14.0"
    },
    {
      "description": "LSP support for Zig via zls.",
      "id": "lsp_zig",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:89efc5642cb9abcd9023c8e5ef3dba85b399884e",
      "version": "0.11.0"
    },
    {
      "description": "Remaps mac modkeys `command/option` to `ctrl/alt`",
      "id": "macmodkeys",
      "mod_version": "3",
      "path": "plugins/macmodkeys.lua",
      "version": "0.1"
    },
    {
      "description": "Add markers to docs and jump between them quickly *([screenshot](https://user-images.githubusercontent.com/3920290/82252149-5faaa200-9946-11ea-9199-bea2efb7ee23.png))*",
      "id": "markers",
      "mod_version": "3",
      "path": "plugins/markers.lua",
      "version": "0.1"
    },
    {
      "description": "Show memory usage in the status view",
      "id": "memoryusage",
      "mod_version": "3",
      "path": "plugins/memoryusage.lua",
      "version": "0.1"
    },
    {
      "description": "Shows a minimap on the right-hand side of the docview. Taken from [@andsve](https://github.com/andsve/lite-plugins/tree/minimap-plugin), and improved upon.",
      "id": "minimap",
      "mod_version": "3",
      "path": "plugins/minimap.lua",
      "version": "0.2"
    },
    {
      "description": "Adds a motion-trail to the caret *([gif](https://user-images.githubusercontent.com/3920290/83256814-085ccb00-a1ab-11ea-9e35-e6633cbed1a9.gif))*",
      "id": "motiontrail",
      "mod_version": "3",
      "path": "plugins/motiontrail.lua",
      "version": "0.2.1"
    },
    {
      "description": "Allows moving back and forward between document positions, reducing the amount of scrolling",
      "id": "navigate",
      "mod_version": "3",
      "path": "plugins/navigate.lua",
      "version": "0.1"
    },
    {
      "description": "Add support for TCP and UDP sockets using SDL_net.",
      "id": "net",
      "mod_version": "3",
      "remote": "https://github.com/jgmdev/lite-xl-net:4ddece50cdc6d00ab09be1896ef0474e89da89b8",
      "type": "library",
      "version": "1.1"
    },
    {
      "dependencies": {
        "font_symbols_nerdfont_mono_regular": {}
      },
      "description": "File icons set for TreeView. Modification of the [nonicons](plugins/nonicons.lua) plugin. Uses [NerdFont](https://www.nerdfonts.com/) icons",
      "id": "nerdicons",
      "mod_version": "3",
      "path": "plugins/nerdicons.lua",
      "type": "plugin",
      "version": "1.2.2"
    },
    {
      "description": "Provides official NodeJs binaries.",
      "id": "nodejs",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-lsp-servers:1df8be64ad87efa45341d8c3953df60d90f9539b",
      "type": "library",
      "version": "20.12.0"
    },
    {
      "dependencies": {
        "font_nonicons": {}
      },
      "description": "File icons set for TreeView. Uses the [Nonicons](https://github.com/yamatsum/nonicons/) font",
      "id": "nonicons",
      "mod_version": "3",
      "path": "plugins/nonicons.lua",
      "version": "0.4.1"
    },
    {
      "description": "Change the opaqueness/transparency of `lite-xl` using shift+mousewheel or a command.",
      "id": "opacity",
      "mod_version": "3",
      "path": "plugins/opacity.lua",
      "version": "0.1"
    },
    {
      "description": "Automatically prompts you if you tried to open a binary file in the editor",
      "id": "open_ext",
      "mod_version": "3",
      "path": "plugins/open_ext.lua",
      "version": "0.1"
    },
    {
      "description": "Opens the parent directory of the current file in the file manager",
      "id": "openfilelocation",
      "mod_version": "3",
      "path": "plugins/openfilelocation.lua",
      "version": "0.1"
    },
    {
      "description": "Opens the selected filename or url",
      "id": "openselected",
      "mod_version": "3",
      "path": "plugins/openselected.lua",
      "version": "0.1"
    },
    {
      "description": "PDF preview for TeX files",
      "id": "pdfview",
      "mod_version": "3",
      "path": "plugins/pdfview.lua",
      "version": "0.1"
    },
    {
      "description": "A plugin manager view for lite-xl that provides GUI access to `lpm`",
      "id": "plugin_manager",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-plugin-manager.git:3e5d5b0827058f2eeddfb166d8128fc086a87e28",
      "version": "0.1"
    },
    {
      "description": "Adds middle mouse click copy/paste (primary selection). To use this plugin, `xclip` must be installed.",
      "id": "primary_selection",
      "mod_version": "3",
      "path": "plugins/primary_selection.lua",
      "version": "0.2"
    },
    {
      "description": "Adds the ability to profile lite-xl with the [lua-profiler](https://github.com/charlesmallah/lua-profiler)",
      "id": "profiler",
      "mod_version": "3",
      "path": "plugins/profiler",
      "version": "0.1"
    },
    {
      "dependencies": {
        "thread": {}
      },
      "description": "Threaded project search with 5-10x better performance.",
      "id": "projectsearch",
      "mod_version": "3",
      "name": "Multithreaded Project Search",
      "remote": "https://github.com/jgmdev/lite-xl-threads:9299a9a6b778cb34b12f0286b9162779920a9197",
      "version": "1.2"
    },
    {
      "description": "Show nesting of parentheses with rainbow colours",
      "id": "rainbowparen",
      "mod_version": "3",
      "path": "plugins/rainbowparen.lua",
      "version": "0.1.1"
    },
    {
      "description": "Allows for you to write a regex and its replacement in one go, and live preview the results.",
      "id": "regexreplacepreview",
      "mod_version": "3",
      "path": "plugins/regexreplacepreview.lua",
      "version": "0.2"
    },
    {
      "description": "Keep a list of recently closed tabs, and restore the tab in order on ctrl+shift+t.",
      "id": "restoretabs",
      "mod_version": "3",
      "path": "plugins/restoretabs.lua",
      "version": "0.1"
    },
    {
      "description": "Displays current scale (zoom) in status view (depends on scale plugin)",
      "id": "scalestatus",
      "mod_version": "3",
      "path": "plugins/scalestatus.lua",
      "version": "0.1"
    },
    {
      "dependencies": {
        "widget": {}
      },
      "description": "Extensible source control management plugin with git and fossil backends.",
      "id": "scm",
      "mod_version": "3",
      "name": "Source Control Management",
      "remote": "https://github.com/lite-xl/lite-xl-scm:930951990f9a3c78178265e5380e3c9e40b109d2",
      "version": "0.1"
    },
    {
      "dependencies": {
        "widget": {}
      },
      "description": "Friendlier search and replace user interface using Widgets.",
      "id": "search_ui",
      "mod_version": "3",
      "path": "plugins/search_ui.lua",
      "version": "0.1"
    },
    {
      "description": "Select a color theme, like VScode, Sublime Text.(plugin saves changes)",
      "id": "select_colorscheme",
      "mod_version": "3",
      "path": "plugins/select_colorscheme.lua",
      "version": "0.1"
    },
    {
      "description": "Highlights regions of code that match the current selection *([screenshot](https://user-images.githubusercontent.com/3920290/80710883-5f597c80-8ae7-11ea-97f0-76dfacc08439.png))*",
      "id": "selectionhighlight",
      "mod_version": "3",
      "path": "plugins/selectionhighlight.lua",
      "version": "0.2"
    },
    {
      "dependencies": {
        "widget": {}
      },
      "description": "Provides a GUI to manage core and plugin settings, bindings and select color theme *([video](https://user-images.githubusercontent.com/1702572/169743674-ececae24-f6b7-4ff2-bfa2-c4762cd327d9.mp4))*. (depends on [`widget`](https://github.com/lite-xl/lite-xl-widgets))",
      "id": "settings",
      "mod_version": "3",
      "path": "plugins/settings.lua",
      "version": "0.7"
    },
    {
      "description": "Displays the current time in the corner of the status view",
      "id": "smallclock",
      "mod_version": "3",
      "path": "plugins/smallclock.lua",
      "version": "0.1"
    },
    {
      "description": "Opens the selected filename or path in project. Useful to open imports.",
      "id": "smartopenselected",
      "mod_version": "3",
      "path": "plugins/smartopenselected.lua",
      "version": "0.1"
    },
    {
      "description": "Smooth caret animation *([gif](https://user-images.githubusercontent.com/20792268/139006049-a0ba6559-88cb-49a7-8077-4822445b4a1f.gif))*",
      "id": "smoothcaret",
      "mod_version": "3",
      "path": "plugins/smoothcaret.lua",
      "version": "0.2"
    },
    {
      "description": "Provides code snippets support",
      "id": "snippets",
      "mod_version": "3",
      "remote": "https://github.com/vqns/lite-xl-snippets:1f0bba02bb3af6df9cb2ce72526851b427caf143",
      "version": "1.1"
    },
    {
      "description": "Sorts selected lines alphabetically",
      "id": "sort",
      "mod_version": "3",
      "path": "plugins/sort.lua",
      "version": "0.2"
    },
    {
      "description": "Sort selected CSS properties alphabetically or using the concentric model. *([demo](https://raw.githubusercontent.com/felixsanz/lite-xl-sortcss/master/demo.gif))*",
      "id": "sortcss",
      "mod_version": "3",
      "remote": "https://github.com/felixsanz/lite-xl-sortcss:ee4552148b38663e24dedcf4bc80ba8221dd54e0",
      "version": "0.1"
    },
    {
      "description": "Underlines misspelt words *([screenshot](https://user-images.githubusercontent.com/3920290/79923973-9caa7400-842e-11ea-85d4-7a196a91ca50.png))* *-- note: on Windows a [`words.txt`](https://github.com/dwyl/english-words/blob/master/words.txt) dictionary file must be placed beside the exe*",
      "id": "spellcheck",
      "mod_version": "3",
      "path": "plugins/spellcheck.lua",
      "version": "0.2"
    },
    {
      "description": "Displays the current date and time in the corner of the status view",
      "id": "statusclock",
      "mod_version": "3",
      "path": "plugins/statusclock.lua",
      "version": "0.1"
    },
    {
      "description": "Keep track of the current scope at the top of the view (*[video](https://user-images.githubusercontent.com/2798487/222133911-e467a583-596c-47ab-8e65-eefc7b5c9112.mp4)*)",
      "id": "sticky_scroll",
      "mod_version": "3",
      "name": "Sticky Scroll",
      "path": "plugins/sticky_scroll.lua",
      "version": "1.0"
    },
    {
      "description": "Save files that require root permissions. Needs `pkexec`.",
      "id": "su_save",
      "mod_version": "3",
      "name": "Save as Super User",
      "path": "plugins/su_save.lua",
      "version": "1.0"
    },
    {
      "description": "Takes an SVG screenshot. Only browsers seem to support the generated SVG properly.",
      "id": "svg_screenshot",
      "mod_version": "3",
      "path": "plugins/svg_screenshot.lua",
      "version": "0.1"
    },
    {
      "description": "Switch between open tabs by searching by name",
      "id": "tab_switcher",
      "mod_version": "3",
      "path": "plugins/tab_switcher.lua",
      "version": "0.3"
    },
    {
      "description": "Displays tab numbers from 1–9 next to their names *([screenshot](https://user-images.githubusercontent.com/16415678/101285362-007a8500-37e5-11eb-869b-c10eb9d9d902.png))*",
      "id": "tabnumbers",
      "mod_version": "3",
      "path": "plugins/tabnumbers.lua",
      "version": "0.1"
    },
    {
      "description": "An integrated terminal for lite-xl. *([screenshot](https://github.com/adamharrison/lite-xl-terminal/assets/1034518/eb8a72a0-ff61-4b95-b009-364ac2725f70))*",
      "id": "terminal",
      "mod_version": "3",
      "remote": "https://github.com/adamharrison/lite-xl-terminal.git:e4dc766969cacfe0be8920f3d859c8e823d0ae6a",
      "version": "1.03"
    },
    {
      "description": "Allows you to play tetris directly in the editor.",
      "id": "tetris",
      "mod_version": 3,
      "path": "plugins/tetris.lua",
      "tags": [
        "game"
      ],
      "version": "0.1"
    },
    {
      "dependencies": {
        "console": {}
      },
      "description": "Compile Tex files into PDF",
      "id": "texcompile",
      "mod_version": "3",
      "path": "plugins/texcompile.lua",
      "version": "0.1"
    },
    {
      "description": "Theme manager with base16 themes",
      "id": "theme16",
      "mod_version": "3",
      "remote": "https://github.com/monolifed/theme16:c39b33cb318d4baa2b4b9cc6e6370cb3ede3ef22",
      "version": "0.1"
    },
    {
      "description": "Select a theme based on filename of active document",
      "id": "themeselect",
      "mod_version": "3",
      "path": "plugins/themeselect.lua",
      "version": "0.1"
    },
    {
      "description": "Titleizes selected string (`hello world` => `Hello World`)",
      "id": "titleize",
      "mod_version": "3",
      "path": "plugins/titleize.lua",
      "version": "0.1"
    },
    {
      "description": "Todo tree viewer for annotations in code like `TODO`, `BUG`, `FIX`, `IMPROVEMENT`",
      "id": "todotreeview",
      "mod_version": "3",
      "remote": "https://github.com/drmargarido/TodoTreeView:e222f7074cae607dec82fb191a4fb077e75886e2",
      "version": "0.1.2"
    },
    {
      "description": "Toggles symbols between `snake_case` and `camelCase`",
      "id": "togglesnakecamel",
      "mod_version": "3",
      "path": "plugins/togglesnakecamel.lua",
      "version": "0.1"
    },
    {
      "description": "Supports spawning true os-threads.",
      "id": "thread",
      "mod_version": "3",
      "name": "Threads",
      "remote": "https://github.com/jgmdev/lite-xl-threads:9299a9a6b778cb34b12f0286b9162779920a9197",
      "type": "library",
      "version": "1.3"
    },
    {
      "description": "Tree-sitter bindings based on `lua-tree-sitter`",
      "id": "tree_sitter",
      "name": "Tree-sitter",
      "remote": "https://github.com/xcb-xwii/lite-xl-tree-sitter:0aafd746d037e8334c26975a61acee7cf1c40da1",
      "type": "library",
      "version": "0.1.0"
    },
    {
      "description": "Extend Lite XL's treeview menu *([screenshot](https://raw.githubusercontent.com/juliardi/lite-xl-treeview-extender/main/screenshot.png))*",
      "id": "treeview-extender",
      "mod_version": "3",
      "remote": "https://github.com/juliardi/lite-xl-treeview-extender:c6583c3b7f726b08f98c1282d03408facd4cbc91",
      "replaces": [
        "treeview-menu-extender"
      ],
      "version": "1.0.0"
    },
    {
      "description": "Displays your current typing speed in characters and words per minute in the status bar",
      "id": "typingspeed",
      "mod_version": "3",
      "path": "plugins/typingspeed.lua",
      "version": "0.1"
    },
    {
      "description": "Allows scrolling outside the bounds of a document",
      "id": "unboundedscroll",
      "mod_version": "3",
      "path": "plugins/unboundedscroll.lua",
      "version": "0.1"
    },
    {
      "description": "Automatically checks for updates and notifies you",
      "id": "updatechecker",
      "mod_version": "3",
      "remote": "https://github.com/vincens2005/lite-xl-updatechecker:3478abd43618da857d4315bcc8fddf08c27e1150",
      "version": "0.1"
    },
    {
      "description": "VI(vim?) bindings with a hint of DOOM Emacs, for lite-xl",
      "id": "lite-xl-vibe",
      "mod_version": "3",
      "name": "vibe",
      "remote": "https://github.com/eugenpt/lite-xl-vibe:651c4fc55a285eeacbf5992b355d9c80f4fc78cb",
      "replaces": [
        "vibe"
      ],
      "version": "0.1"
    },
    {
      "description": "Audio visualizer for Lite XL",
      "id": "visu",
      "mod_version": "3",
      "remote": "https://github.com/TorchedSammy/Visu:782c7b1ebde38dad2c3c6a1c1dee6761230dea16",
      "version": "0.1"
    },
    {
      "description": "Plugin library that provides a set of re-usable components to more easily write UI elements for your plugins",
      "id": "widget",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-widgets:61789bdc87d52ba7838927af9aa2671efdcea185",
      "type": "library",
      "version": "0.2.1"
    },
    {
      "description": "Adds in a word count to the statusview.",
      "id": "wordcount",
      "mod_version": "3",
      "path": "plugins/wordcount.lua",
      "version": "0.1"
    },
    {
      "description": "A simple library that provides a web client to fetch, and submit data via HTTP requests.",
      "id": "www",
      "mod_version": "3",
      "remote": "https://github.com/adamharrison/lite-xl-www.git:aecc866c33b0a954fbd41a5de2842bb97f4ac45e",
      "type": "library",
      "version": "0.1"
    },
    {
      "description": "`pywal` integration with Lite XL.",
      "id": "wal",
      "mod_version": "3",
      "name": "wal.lxl",
      "remote": "https://github.com/ThaCuber/wal.lxl:93bcf5dd64e9945fa88de22cd8b62a5467e80d6a",
      "replaces": [
        "wal_lxl"
      ],
      "version": "1.1"
    },
    {
      "description": "A metapackage that provides the minimum functionalities for Lite XL to run as a minimal IDE.",
      "id": "ide",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for C/C++.",
      "id": "ide_c",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for Java.",
      "id": "ide_java",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for Javascript.",
      "id": "ide_javascript",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for Lua.",
      "id": "ide_lua",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for Python.",
      "id": "ide_python",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for Rust.",
      "id": "ide_rust",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for TeX.",
      "id": "ide_tex",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for Typescript.",
      "id": "ide_typescript",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    },
    {
      "description": "A metapackage that turns Lite XL into an IDE for Zig.",
      "id": "ide_zig",
      "mod_version": "3",
      "remote": "https://github.com/lite-xl/lite-xl-ide.git:3bb229a5b7ed7028c40bba831b0dd77b06c3e262",
      "version": "0.1"
    }
  ],
  "remotes": [
    "https://github.com/lite-xl/lite-xl-lsp-servers:latest",
    "https://github.com/lite-xl/lite-xl-ide:latest"
  ]
}"#;
