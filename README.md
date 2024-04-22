# Lite-xl Plugins lab's website's source code
---
## Important details about the src code:
- this website is using the [rohanasan backend framework](https://github.com/rohanasan/rohanasan-rs)
- the backend is majourly handleing the serving part of the website.
- Code structuring is majourly based on rohanasan-rs's default folder configuration which includes the following:
    - the src directory contains the rust code
    - the static folder contains the static items that need to be sent to client at the route static/
    - the html folder contains the html files that are sent according to the main.rs file.
    - the dockerfile contains the source code that is executed on the render.com.
    - the cargo.toml file contains the nessecary libraries required to build the backend framework.
    - Inside the static folder, you can see a main.ts file and a main.js file used only for the @plugins page.
    - You can edit the main.ts and compile it to main.js using npm's tsc.
### Why rohanasan backend framework?
- to ensure speed, stability, and easiness all at the same time.
### Why type script instead of javascript directly?
- to ensure code's readability, and also to add some strictness to how the code is typed to avoid any possible errors.
---
# Contribution
feel free to fork this repo/open a pr/ open an issues.
