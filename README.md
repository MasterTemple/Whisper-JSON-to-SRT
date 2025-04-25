# WhisperX JSON to SRT Converter

This is meant to convert unaligned `.json` transcripts from [whisperX](https://github.com/m-bain/whisperX) (or any compatible JSON) into `.srt` files.

While WhisperX has the ability to output `.srt` files, I don't want to have to store both.

# Usage

This has 2 use cases

## 1. Inside Vim

If I have a `.json` file opened in Vim, I can just do as follows

```vim
:%!wjs
```

and the buffer is replaced with the `.srt` contents.

## 2. Command Line Application

Specify the input `.json` file and output `.srt` file

```bash
wjs -j input.json -s output.srt
```

# Installation

```bash
# Install for local usage
cargo install --git https://github.com/MasterTemple/whisper-json-to-srt
# Rename for ease of use
ln ~/.cargo/bin/whisper_json_to_srt ~/.cargo/bin/wjs
```

# Example

Convert from

```json
{
  "segments": [
    {
      "text": " Good morning. Please open your Bibles with me to the end of 1st Peter, 1st Peter chapter 5. This morning and next week, Lord willing, we will conclude our study of 1st Peter, which by that time will have spanned 68 sermons and began in the summer of 2021, which is kind of a long time. I",
      "start": 0.031,
      "end": 29.967
    },
    // ..
  ]
}
```

into

```srt
1
00:00:00,031 --> 00:00:29,967
Good morning. Please open your Bibles with me to the end of 1st Peter, 1st Peter chapter 5. This morning and next week, Lord willing, we will conclude our study of 1st Peter, which by that time will have spanned 68 sermons and began in the summer of 2021, which is kind of a long time. I

```

# Extra

I transcribe files with this command:

```bash
alias wx="whisperx --no_align --language en --model turbo --output_format json"
```

I can also view the JSON as text with:

```vim
" Get just text
:%!jq .segments[].text -r
" Trim whitespace
:%s/\v(^\s+|\s\+$)//eg
" Make text more readable
:norm vipgq
```