// Import and export data from other modules defined in the profile toml.
import "urls" as URLS;
import "factorial" as fac;

// Prints text.
print(
    "If this is your first time trying the example and you see a panic, \
    you will need to change the path in ~/.config/reconf/modules/readme/readme.rhai!"
);

let factorials = "";

// Calculate factorials 1 - 9. (Max recursion is 8 by default in debug)
for i in 1..9 {
    if i > 1 {
        factorials += ", "
    }

    factorials += fac::factorial(i).to_string();
}

// NOTE: template does NOT write to the path when calling this function. The path gets cached along
// with the template name and data until all modules have finished executing. This caching step is
// done to prevent calling template twice for the same path because there's no guarantee which order
// they will execute in. Calling template twice for the same path causes a panic.
// USAGE: template("/absolute/path", "template", #{ ... });
template("README.md", "readme", #{
    rhai_url: URLS::rhai,
    hb_url: URLS::handlebars,
    sum: 1 + 1,
    facs: factorials,
    sum_ex: "1 + 1 = {{sum}}",
    hex_ex: `{{hex_to_rgb "#AABBCC"}}`,
});

// Use shell commands.
command("echo");
command("echo", ["echo does not work in rhai, use print!"]);
