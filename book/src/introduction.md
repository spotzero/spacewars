# Introduction

In this set of tutorials. I will demonstrate how to develop a full and complete game using Amethyst.

I'll be implementing the classic game Spacewar! which I have fond memories of playing as a child. My implementation will of course have some modern spice thrown on it.

## About me

I'm an experienced software developer and architect, but fairly new to Rust and have never used the Amethyst engine before.

I decided the implement Spacewar! to learn how to use Amethyst. Since there seems to be a real gap in tutorials showing how to make complete games with Amethyst, I decided to document my adventure as a tutorial for others wanting to do the same.

Hopefully I can leverage my inexperience with Amethyst to comes out more on the side of "I have the same questions as any one using this tutorial, and so it will provide a short-cut to learning the system" and not come out "blind leading the blind".

## Assumptions about you

I'm going to assume reader understand basic Rust and have read the Amethyst book, tried a few of the examples, maybe looks at the pong example, but that's it.

I'm not going to talk about how the specs ECS works or the struture of Amethyst. Honestly, the Amethyst book does a great job of this.  I want to fill the gap you get to after the Amethyst book of how to leverage it to develop a complete game.