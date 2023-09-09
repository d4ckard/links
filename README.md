# Links

I love to collect bookmarks in my browser of choice of high-quality, interesting, and generally useful writing I find on the web.

To me, they feel very precious, and I want to have **full control** over what happens to them.

Keeping them only in my browser is therefore not an option. Sticking to this hacker ethos of mine, I also decided to not use any popular file format for storing structures data like this (e.g. JSON, TOML, YAML, etc.).

All of them are both overkill and too complicated for me to be certain, that I can grasp and control every single one of their details. I want to be able to parse and process my bookmarks without all the overhead connected to them.

Choosing which format to use, I was careful not to fall into the *diy-just-worse* trap.

The design I landed on looks as follows:

1. In a links file, every line is an atomic unit.
2. The first line in a links file is a bookmark's title (a *title line*).
3. Each *title line* is followed by another line that represents the bookmark's URL. (a *URL line*).
4. Each *URL line* is followed either by another *title line* or the end of the file.

As you can see, this is brutally simple to parse and only uses newline characters, which don't appear in URLs and are not needed in page titles. You could say this is like CSV with commas replaced by newlines and two fixed columns, which eliminate the need for a special row delimiter.

So far, I have only written a pretty-printer for this format. [As you can see,](links/src/main.rs) it neatly fits onto a single page of a screen. From here, it's possible to quickly add any utility that's desired.
