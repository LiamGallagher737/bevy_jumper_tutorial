export const SITE = {
	title: 'Bevy Jumper Tutorial',
	description: 'A tutoial on how to create a jumper game with Bevy and Rust.',
	defaultLanguage: 'en_US',
};

export const OPEN_GRAPH = {
	image: {
		src: 'https://github.com/withastro/astro/blob/main/assets/social/banner.jpg?raw=true',
		alt:
			'astro logo on a starry expanse of space,' +
			' with a purple saturn-like planet floating in the right foreground',
	},
	twitter: 'astrodotbuild',
};

export const KNOWN_LANGUAGES = {
	English: 'en',
};

// Uncomment this to add an "Edit this page" button to every page of documentation.
export const GITHUB_EDIT_URL = `https://github.com/LiamGallagher737/bevy_jumper_tutorial/tree/master/bevy_jumper_site/`;

// Uncomment this to add an "Join our Community" button to every page of documentation.
// export const COMMUNITY_INVITE_URL = `https://astro.build/chat`;

// Uncomment this to enable site search.
// See "Algolia" section of the README for more information.
// export const ALGOLIA = {
//   indexName: 'XXXXXXXXXX',
//   appId: 'XXXXXXXXXX',
//   apiKey: 'XXXXXXXXXX',
// }

export const SIDEBAR = {
	en: [
		{ text: '', header: true },
		{ text: 'Getting Started', header: true },
		{ text: 'Introduction', link: 'getting-started/introduction' },
		{ text: 'Setting up Rust', link: 'getting-started/rust-setup' },
		{ text: 'Setting up Bevy', link: 'getting-started/bevy-setup' },
		{ text: 'Setting up Assets', link: 'getting-started/asset-setup' },
		{ text: 'Creating a Level', header: true },
		{ text: 'Setting up a Camera', link: 'level-creation/camera-setup' },
		{ text: 'Adding a Background', link: 'level-creation/background' },
	],
};
