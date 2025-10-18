const blogpostMarkdown = `# control

*humans should focus on bigger problems*

## Setup

\`\`\`bash
git clone git@github.com:anysphere/control
\`\`\`

\`\`\`bash
./init.sh
\`\`\`

## Folder structure

**The most important folders are:**

1. \`vscode\`: this is our fork of vscode, as a submodule.
2. \`milvus\`: this is where our Rust server code lives.
3. \`schema\`: this is our Protobuf definitions for communication between the client and the server.

Each of the above folders should contain fairly comprehensive README files; please read them. If something is missing, or not working, please add it to the README!

Some less important folders:

1. \`release\`: this is a collection of scripts and guides for releasing various things.
2. \`infra\`: infrastructure definitions for the on-prem deployment.
3. \`third_party\`: where we keep our vendored third party dependencies.

## Miscellaneous things that may or may not be useful

##### Where to find rust-proto definitions

They are in a file called \`aiserver.v1.rs\`. It might not be clear where that file is. Run \`rg --files --no-ignore bazel-out | rg aiserver.v1.rs\` to find the file.

## Releasing

Within \`vscode/\`:

- Bump the version
- Then:

\`\`\`
git checkout build-todesktop
git merge main
git push origin build-todesktop
\`\`\`

- Wait for 14 minutes for gulp and ~30 minutes for todesktop
- Go to todesktop.com, test the build locally and hit release
`;

let currentContainer: HTMLElement | null = null;
// Do not edit this method
function runStream() {
	currentContainer = document.getElementById("markdown")!;

	// This randomly split the markdown into tokens between 2 and 20 characters long
	// simulates the behavior of an ml model that's giving you weirdly chunked tokens
	const tokens: string[] = [];
	let remainingMarkdown = blogpostMarkdown;
	while (remainingMarkdown.length > 0) {
		const tokenLength = Math.floor(Math.random() * 18) + 2;
		const token = remainingMarkdown.slice(0, tokenLength);
		tokens.push(token);
		remainingMarkdown = remainingMarkdown.slice(tokenLength);
	}

	const toCancel = setInterval(() => {
		const token = tokens.shift();
		if (token) {
			addToken(token);
		} else {
			clearInterval(toCancel);
		}
	}, 20);
}

type GlobalState = {
	open_block: boolean;
	buffer: string;
	heading_level: number;
	inside_code_block: boolean;
	code_block_lang: string;
};

const state: GlobalState = {
	open_block: false,
	buffer: "\n",
	heading_level: 0,
	inside_code_block: false,
	code_block_lang: "",
};

function addToken(chunk: string) {
	if (!currentContainer) {
		throw new Error("No current container set");
	}

	chunk = chunk.replaceAll("\r\n", "\n");

	for (const char of chunk) {
		const { outcome, elements } = parse_markdown(char);
		if (outcome === ParserOutcome.Continue) {
			continue;
		} else if (outcome === ParserOutcome.FlushToNewElement) {
			const { root, inner } = createNestedElements(elements);
			inner.textContent = char;
			currentContainer.appendChild(root);
		} else if (outcome === ParserOutcome.FlushToLastElement) {
			const last = currentContainer.lastElementChild as HTMLElement;
			last.textContent += char;
		}
	}
}

function createNestedElements(tags: string[]): {
	root: HTMLElement;
	inner: HTMLElement;
} {
	let root = null;
	let current = null;

	for (const tag of tags) {
		const el = document.createElement(tag);
		if (!root) {
			root = el;
		} else {
			current!.appendChild(el);
		}
		current = el;
	}

	return { root: root!, inner: current! };
}

enum ParserOutcome {
	Continue,
	FlushToNewElement,
	FlushToLastElement,
}

function parse_markdown(char: string): {
	outcome: ParserOutcome;
	elements: string[];
} {
	// HEADINGS
	if (
		(char === "#" && state.buffer.at(-1) === "\n") ||
		(char === "#" && state.buffer.at(-1) === "#")
	) {
		state.heading_level++;
		return {
			outcome: ParserOutcome.Continue,
			elements: [""],
		};
	} else if (char === " " && state.heading_level > 0 && !state.open_block) {
		state.open_block = true;
		return {
			outcome: ParserOutcome.FlushToNewElement,
			elements: [`h${state.heading_level}`],
		};
	} else if (char === " " && state.heading_level > 0 && state.open_block) {
		return {
			outcome: ParserOutcome.FlushToLastElement,
			elements: [""],
		};
	} else if (char !== "\n" && state.heading_level > 0) {
		return {
			outcome: ParserOutcome.FlushToLastElement,
			elements: [""],
		};
	} else if (char === "\n" && state.heading_level > 0) {
		state.open_block = false;
		state.heading_level = 0;
		state.buffer = "\n";

		return {
			outcome: ParserOutcome.FlushToNewElement,
			elements: ["p"],
		};
	}

	// CODE BLOCKS
	if (state.buffer === "\n```" && char !== "\n" && !state.inside_code_block) {
		state.code_block_lang += char;
		return {
			outcome: ParserOutcome.Continue,
			elements: [""],
		};
	} else if (
		state.buffer === "\n```" &&
		char === "\n" &&
		!state.inside_code_block
	) {
		state.inside_code_block = true;
		return {
			outcome: ParserOutcome.FlushToNewElement,
			elements: ["pre", "code"],
		};
	} else if (state.inside_code_block) {
		return {
			outcome: ParserOutcome.FlushToLastElement,
			elements: [""],
		};
	} else if (
		state.buffer === "\n```" &&
		char !== "\n" &&
		state.inside_code_block
	) {
		state.inside_code_block = false;
		state.code_block_lang = "";
		return {
			outcome: ParserOutcome.FlushToNewElement,
			elements: ["p"],
		};
	}

	state.buffer += char;
	if (char === "\n") {
		state.buffer = "\n";
	}

	return {
		outcome: ParserOutcome.FlushToLastElement,
		elements: [""],
	};
}
