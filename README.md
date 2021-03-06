# emorand *(just a 🎲 emoji)*

Prints a random emoji to stdout.

> *"Why?"* they asked.

I wanted a random emoji in my bash prompt, and I didn't want to save a subset of "acceptable" emojis to pick from with `$RANDOM` into by `.bashrc`. I wanted all\* of the emojis, a surprise at every prompt. I'm wild like that.

> *"But why Rust?"* they asked, unquestionably inquisitively.

Because Rust is awesome and was, this time at least, more satisfying than bash scripting.

### 📖 Usage

    $ emorand
    👏

The first time you run it might take a little bit longer: it will build a cache from [the online list of emoji sequences](https://unicode.org/Public/emoji/13.1/emoji-sequences.txt). After that, it should be *pretty fast*.

If you want it in your bash prompt like I do, you'll need to do some `~/.bashrc` magic. If you want a new emoji *per terminal,* use:

    PS1="$(emorand) > "

If you feel really fancy (like I do), you may define `PROMPT_COMMAND` instead to get a new emoji at *every prompt*:

    emorand_prompt() { PS1="$(emorand) > "; }
    PROMPT_COMMAND=emorand_prompt

If you, too, like to have git info and other stuff like that in your prompt, you probably already have such a function, which you may simply adjust.

### 🔧 Build and install

As of this writing, emorand has yet to be published to distribution repositories or even crates.io (I know, shocking). Until this blessed day comes, you may build and install emorand from source with:

    $ cargo build --release

How and where you install it then is up to you. I, for one, have chosen to simply use `install`...

    $ sudo install -m0755 target/release/emorand /usr/local/bin

... but `cargo install` is also a very good option (if you can figure out the right options).

### 🪲 Caveats

\* Ok, so I lied, it doesn't actually print *any* emoji. Those that are defined by unicode sequences (eg. skin tones, flags, ...) will never show up. Two reasons:

- My terminal can't handle some of those.
- emorand uses a cache, which I'm afraid would get bigger than is reasonable if I included those (then again, disk space is cheap...)

Maybe I'll try to figure out a way to fix this later ;-)

### ⚖️ License

emorand is released under the terms of the AGPL 3.0 (or later).
