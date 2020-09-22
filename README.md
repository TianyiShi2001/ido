# ido

`ido` means "I do...".

This is a re-implementation of the task tracker [**carpe-diem**](https://github.com/TianyiShi2001/carpe-diem) in Rust. I used Typescript to wrote the original project and ran into the callback hell.

I have just started this project for a few days so breaking changes are expected. Use it at your own risk.

# Installation

Currently, only `cargo install` is supported.

```
cargo install ido
```

# Usage (2020-09-20)

- execute `ido` in a terminal to launch the app.
- if it's the first time `ido` is being used, you should be prompted to specify the directory at which your data are stored. Enter the full path, such as `/Users/tianyishi/Documents/ido`
- Press `n` to track a new task
- Press `q` to finish
- You should see `log.json` at the directory you specified.

# Philosophy

You may refer to the original project, [**carpe-diem**](https://github.com/TianyiShi2001/carpe-diem), for a more comprehensive description.

In short, `ido` will faithfully record what you have done (task, time, duration, efficiency, custom attributes) in future-proof json files. You can store them wherever you want and use them easily in your workflow (you may do a data analysis on your own behaviour, or visualize what you have done on a calendar and share with others, for example). Many fancy and expensive iPad apps won't allow you to do so.

# Compatibility

Currently only compatible on Linux and macOS

# Roadmap

- [ ] Efficiency
- [ ] Menu bar (new task|view|settings)
- [ ] Notes/Custom attributes
- [ ] add past events
- [ ] set goal + progress bar