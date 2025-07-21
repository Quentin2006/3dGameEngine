# 3d Game Engine

## 🎯 GOALS

- Learn modern rust programming
- Understand the basics of graphics programming
- Develop a basic 3d game engine that can render obj files

## 🗂 LAYOUT

- I will do periodic updates at the end of each session to talk about what I
  learned, and what I'm working on.
- I plan to do weekly check-in's every Sunday, this will summarize what I
  learned, and what I'm working on for the whole week.

## 📅 Day 0

This is just a small little summer project. I want to learn some rust
programming, so I've decided to make a 3d render.

I plan to do weekly updates, and I'm deeming this "Day 0". This is mainly so
I can be held accountable for working on this project,
and make it easier for me to start.

### 🔜 Week 1 hopes

- Learn the basics of Rust

- Set up a Rust project and get it compiling

- Try rendering something to the screen

## 📅 Week 1

This week, I covered some of the rust docs, just enough to actualy get started.
I made very good progress today, I implemented a very crude implementaion
of a rasterizer.

I will improve this, but currently you can add a triangle
into world space and it renders!

The last thing I was working on was getting barycentric interpolation working,
but for some reason v2 is showing as green, when it should be blue.
So I got to look into that a bit more.

### 🔜 Week 2 hopes

- Finish barycentric interpolation

- Read more of the Rust docs

- Clean up the code using operator overloading

- Add a z-buffer for proper depth handling

## 📅 Week 2 (ish)

I got a little busy with work and life, so I couldnt find the time to
work on this last week or this week. I did do one small bug fix,
so now, the triangle actualy gets rendered proper on screen! The issue
was in the draw function. I was acessing the indicies in my raster
improperly.

### Week 3 hopes (same as week 2 hopes)

- Finish barycentric interpolation

- Read more of the Rust docs

- Clean up the code using operator overloading

- Add a z-buffer for proper depth handling
