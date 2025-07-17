
const post_container = document.querySelector("#blog_posts");

// Takes an array of filters and manually creates a query string from it.
// Qs.stringify uses an array notation that's incompatible with serde_qs,
// on the Rust-server side.
function filters_to_string(filters) {
	var result = "";
	for (var i = 0; i < filters.length; i++) {
		result += `filters[${i}]=${filters[i]}&`;
	}
	return result;
}

const LOADER_HTML = "<progress class=\"circle large\"/>";
const MSECS_BEFORE_LOADER = 200;
function show_loader() {
	post_container.innerHTML = LOADER_HTML;
}

async function toggle_tag_filter(tag, _) {
	var query = Qs.parse(location.search, { ignoreQueryPrefix: true });
	if (query.filters == null) {
		query.filters = [tag];
	} else {
		var tag_index = query.filters.indexOf(tag);
		if (tag_index == -1) {
			query.filters.push(tag);
		} else {
			query.filters.splice(tag_index, 1);
		}
	}
	const filter_string = filters_to_string(query.filters);
	const timeout_before_loader = window.setTimeout(show_loader, MSECS_BEFORE_LOADER);
	const blog_posts = await fetch("/blog_post_list?" + filter_string);
	if (!blog_posts.ok) {
		post_container.innerHTML = "Error: " + blog_posts.statusText;
		return;
	}
	post_container.innerHTML = await blog_posts.text();
	window.clearTimeout(timeout_before_loader);
	bind_tag_buttons_on_click();
	var current_url = new URL(location);
	current_url.search = filter_string;
	history.replaceState(null, "", current_url.toString());
}

function bind_tag_buttons_on_click() {
	var tag_btns = document.querySelectorAll("#filter_tag");
	tag_btns.forEach((btn) => {
		btn.onclick = toggle_tag_filter.bind(null, btn.dataset.tag);
	});
}

window.onload = bind_tag_buttons_on_click;
