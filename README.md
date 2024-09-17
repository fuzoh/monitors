# Monitors

**v0.2.0 :** [Read changelog](./CHANGELOG.md) <!-- x-release-please-version -->

> An ultra-light, ultra simple binary to change monitor configuration in [hyprland](https://hyprland.org/).
> Execute it on monitor plug/unplug or on shortcut.

## Todo's

- [x] Retrieve list of active monitors (name, resolution)
- [ ] Propose default possible configurations, inferred from monitors available.
- [ ] Check if configuration is compatible, apply it
- [ ] Add parameters to scope the type of proposed configurations to specific needs.

## new approach

To be much simpler, I want to massively simplify my initial idea.

From now, the chosen way was to develop a small ratatui terminal ui,
with monitor selection. Then, a simple hyprland script will trigger this one on monitor connection.

I realise that this is a complex approach in response of the simple initial goals.

Finally, I will take another direction. As Linux philosophy "small tools that do one thing well".

I want to be able to use tools like [dmenu](https://github.com/aario/dmenu), in my case, I love [tofi](https://github.com/philj56/tofi).

So, this new approach will be much simpler. I want to create a light cli binary, that print monitor options into stdout, so you can pipe it to tofi, and that can take options in stdin to determine the changes to be made on hyprctl.

With this approach, you can use a simple pipe chain between monitors and tofi.
