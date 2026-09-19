import path from "path";
import fs from "fs";
import Image from "@11ty/eleventy-img";

export default function (eleventyConfig) {
	// Plain-text excerpt of rendered page content, for og:description on
	// posts that don't declare `description` in front matter. Strips the
	// post layout's own chrome (title, date/tags) and non-prose blocks
	// before taking the first ~160 characters.
	eleventyConfig.addFilter("ogExcerpt", function (content) {
		let text = (content || "")
			.replace(/<h1[\s\S]*?<\/h1>/, "")
			.replace(/<ul class="post-metadata">[\s\S]*?<\/ul>/, "")
			.replace(/<pre[\s\S]*?<\/pre>/g, " ")
			.replace(/<figure[\s\S]*?<\/figure>/g, " ")
			.replace(/<[^>]+>/g, " ")
			.replace(/\s+/g, " ")
			.replace(/\s+([,.;:!?)’”])/g, "$1")
			.replace(/([(‘“])\s+/g, "$1")
			.trim();
		if (text.length <= 160) {
			return text;
		}
		return text.slice(0, 160).replace(/\s+\S*$/, "") + "…";
	});

	// Resolve a front-matter image (`image` or legacy `coverImage`) to an
	// absolute URL for og:image, generating a 1200px-wide JPEG alongside
	// the page. Paths are relative to the post directory; a bare filename
	// is also tried under the post's images/ directory; a leading slash
	// resolves under public/.
	eleventyConfig.addAsyncFilter("ogImage", async function (src, siteUrl) {
		const page = this.page;
		let candidates;
		if (src.startsWith("/")) {
			candidates = [path.join("public", src)];
		} else {
			const dir = path.dirname(page.inputPath);
			candidates = [path.join(dir, src), path.join(dir, "images", src)];
		}
		const input = candidates.find((p) => fs.existsSync(p));
		if (!input) {
			console.warn(`[opengraph] og:image not found for ${page.inputPath}: ${src}`);
			return "";
		}
		const stats = await Image(input, {
			widths: [1200],
			formats: ["jpeg"],
			outputDir: path.join("_site", page.url),
			urlPath: page.url,
		});
		return new URL(stats.jpeg[0].url, siteUrl).href;
	});
}
