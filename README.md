# Commitr

My personal first CLI in RUST. CommitR is able to commit added files and add the modification scope in every commit body.

## How to use 
```
git add <PATH>
./commitr "feat: commit message" 
```

Result in git log : 

```
Author: Commitr <commitr@commitr.com>
Date:   Wed Apr 27 20:07:25 2022 +0200

    feat: commit message
    
    modified:  : src/main.rs
    delete:  : src/useless_file.rs
    created:  : src/the_best_feature.rs
```

