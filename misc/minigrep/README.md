# Minigrep

Simple, small grep like utility developed while learning rust.
Brings with it a small poem for testing.

## Sample run on command line

```bash
$ cat <<EOF >poem.txt
I'm Nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
EOF

# Case sensitive search
$ cargo run to poem.txt
poem.txt:2: Are you nobody, too?
poem.txt:6: How dreary to be somebody!

# Disable case sensitivity
$ MINIGREP_ICASE=true cargo run nobody poem.txt
poem.txt:1: I'm Nobody! Who are you?
poem.txt:2: Are you nobody, too?
```

