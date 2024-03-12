# supergitignore

WARNING: In previous versions, there were a few accidental ignores that were not supposed to be there and may have caused damage to some people. As a result, I have removed all of them. I apologize for any inconvenience this may have caused and will continue working on the .gitignore file to ensure that there are no other errors that I might have missed. If you spot anything yourself feel absolutely free to create pull requests on the GitHub repo and I will merge as soon as possible

I always create new projects and until now I used DOOM emacs for creating my `.gitignore` files.
DOOM emacs (a distribution of emacs) has this nice built-in feature that if you
create a new `.gitignore` file with it, there would be a few suggestions that would show up
on things that it can put in there for you, but there are 2 main problems with that:

1. Currently these are just simple and small gitignores, but I prefer a massive file that I could just generate once, and then generally just forget about.
2. It takes time for me to open up DOOM emacs, go to my project's directory, create a new `.gitignore` file, save and quit, push the changes, and then finally commit them.

That's why I created supergitignore, you just run it, and it generates a massive gitignore file for you with all the common stuff that you'll need already in it.

I also decided to publish it here, so that other people like you can enjoy this as well. So have fun :)

To install supergitignore, run the `cargo install supergitignore` command in your terminal (you need cargo installed for this)

To run it, simply run the `supergitignore` command in your project's directory, and everything should work!
