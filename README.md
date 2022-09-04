# Annie-Bunny, Wix- custom search engine
A [Rust](https://github.com/rust-lang/rust) repo for the Annie-Bunny (ab) search engine.
Built fully with [Rust](https://github.com/rust-lang/rust).
<br/>
Thanks to Meta developers article [here](https://developers.facebook.com/blog/post/2020/06/03/build-smart-bookmarking-tool-rust-rocket/) that is a Rust+Rocket Smart bookmarking tool based on the open sourced [bunny1](https://github.com/ccheever/bunny1).
## Commands
- `gh <OPTIONAL_QUERY>` / -> will resolve to either `https://github.com` or search GitHub for `<OPTIONAL_QUERY>`, adding `@` will resolve to `https://github.com/<OPTIONAL_QUERY>`
- `ghx <OPTIONAL_QUERY>` -> searches the entire [Wix repo's](https://github.com/search?q=org%3Awix+org%3Awix-p+org%3Awix-private+org%3Awix-playground+org%3Awix-platform+org%3Awix-system+org%3Awix-incubator) (other than "Archive") for the query.
- `wbo <OPTIONAL_QUERY>` -> searches [Wix Back-office (bo.wix)](https://bo.wix.com) with the given query
- `wl <OPTIONAL_QUERY>` / `wl @<USER_PROFILE_ID>` -> searches [wix-life](https://wix-life.com) with the given query, given `@` will go for the given users' profile
- `cal <OPTIONAL_QUERY>` / `cal ev` -> will resolve to either Google Calendar or search `<OPTIONAL_QUERY>` within it. Using `ev` will result in a new calendar event.
- Any other query that will not contain any of the above commands will resolve to a regular google search.

## Installation
### Chrome
1. Go to [Chrome's search engine settings](chrome://settings/searchEngines)
2. Click the [Add]() button under `Site search`
3. Create a new search engine with the following attributes:
   - Search Engine: `Annie Bunny`<br/>
   - Shortcut: `ab`<br/>
   - URL with %s in place of query: `https://fast-harbor-72501.herokuapp.com/search?q=%s`
### Firefox
1. Install the “Add custom search engine” Firefox Add-on
2. Open up the extension
3. Fill out the form with the following values:
   - Name: `Annie Bunny`
   - Search URL: `https://fast-harbor-72501.herokuapp.com/search?q=%s`
4. Click “Add custom search engine”
5. Check the box “Make this the current search engine”
6. Click “Add”
