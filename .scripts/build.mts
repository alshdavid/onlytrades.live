import path from "node:path";
import fs from "node:fs";
import url from "node:url";
import ejs from "ejs";
import * as sass from "sass";
import * as yaml from "yaml";
import { Paths } from "./paths.mts";
import { rspack, type RspackOptions } from "@rspack/core";

const assets = [
  [Paths["~/root/static/"]("assets"), Paths["~/root/dist/"]("assets")],
];

const styles = [
  [
    Paths["~/root/static/"]("styles.scss"),
    Paths["~/root/dist/"]("styles.[contenthash].css"),
  ],
];

const entries = new Map<string, string>();

type Route = {
  path: string | string[];
  target: string;
  dest: string;
};
const routeTable: Array<Route> = yaml.parse(
  fs.readFileSync(Paths["~/root/static/"]("routes.yaml"), "utf8"),
);

const html: Array<[string, string]> = [];
const manifest: Record<string, string> = {}

for (const route of routeTable) {
  const source = route.target;
  const paths = Array.isArray(route.path) ? route.path : [route.path];
  const dest = route.dest.replace('./', '')

  for (let routePath of paths) {
    manifest[routePath] = dest
  }

  html.push([
    Paths["~/root/static/"](source),
    Paths["~/root/dist/"](dest),
  ]);
}

await fs.promises.rm(Paths["~/root/dist"], { recursive: true, force: true });

const rspackConfig: RspackOptions = {
  mode: "production",
  target: ["web", "es2022"],
  entry: Paths["~/root/static/"]("main.ts"),
  output: {
    path: Paths["~/root/dist"],
    filename: "bundle.[contenthash].js",
    module: true,
    chunkFormat: "module",
    chunkLoading: "import",
    workerChunkLoading: "import",
  },
  resolve: {
    extensions: ["...", ".ts", ".tsx", ".jsx"],
    extensionAlias: {
      ".js": [".ts", ".tsx", ".js"],
    },
    alias: {
      "preact": Paths["~/root/"]('node_modules', 'preact', 'dist', 'preact.module.js'),
      'preact/hooks': Paths["~/root/"]('node_modules', 'preact','hooks','dist','hooks.module.js'),
      react: "preact/compat",
      "react-dom/test-utils": "preact/test-utils",
      "react-dom": "preact/compat",
      "react/jsx-runtime": "preact/jsx-runtime",
    },
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        use: {
          loader: "builtin:swc-loader",
          options: {
            target: "es2024",
            jsc: {
              parser: {
                syntax: "ecmascript",
                jsx: true,
                decorators: true,
              },
              transform: {
                react: {
                  pragma: "h",
                  pragmaFrag: "Fragment",
                },
                decoratorVersion: "2022-03",
                decoratorMetadata: true,
              },
            },
          },
        },
        type: "javascript/auto",
      },
      {
        test: /\.tsx?$/,
        use: {
          loader: "builtin:swc-loader",
          options: {
            target: "es2024",
            jsc: {
              parser: {
                syntax: "typescript",
                tsx: true,
                decorators: true,
              },
              transform: {
                react: {
                  pragma: "h",
                  pragmaFrag: "Fragment",
                },
                decoratorVersion: "2022-03",
                decoratorMetadata: true,
              },
            },
          },
        },
        type: "javascript/auto",
      },
    ],
  },
};

await new Promise<Record<string, string>>(
  (res, rej) => {
    rspack(rspackConfig, (err, stats) => {
      if (err) {
        console.error("Fatal Rspack error:", err);
        return rej(err);
      }

      if (!stats) {
        console.error("Fatal Rspack error: NoStats");
        return rej(new Error("NoStats"));
      }

      if (stats.hasErrors()) {
        console.error(stats.toString({ colors: true }));
        return rej(new Error("Compilation Errors"));
      }

      console.log(
        stats.toString({
          chunks: false, // Less noise; don't show individual chunk breakdowns
          modules: false, // Don't list every single source file bundled
          colors: true, // Beautiful terminal colors (highly recommended!)
          assets: true, // Show the generated files and sizes
          timings: true, // Show how long the build took
        }),
      );

      const statsJson = stats.toJson({
        all: false,
        entrypoints: true,
        chunks: true,
        chunkOrigins: true, // Crucial to find the absolute source paths
      });

      const mapping: Record<string, string> = {};

      if (statsJson.entrypoints) {
        for (const [entryName, entryInfo] of Object.entries(
          statsJson.entrypoints,
        )) {
          const outputAsset = entryInfo.assets?.[0]?.name;

          if (outputAsset) {
            const matchingChunk = statsJson.chunks?.find((c) =>
              c.names?.includes(entryName),
            );

            const fullSourcePath = matchingChunk?.origins?.[0]?.request;

            if (fullSourcePath) {
              mapping[fullSourcePath] = outputAsset;
            } else {
              mapping[entryName] = outputAsset;
            }
          }
        }
      }

      for (const [key, value] of Object.entries(mapping)) {
        manifest[`/${value}`] = value
        entries.set(key, `/${value}`);
      }
      res(mapping);
    });
  },
);

console.log("Build Styles");
for (const [entryPath, distPath] of styles) {
  const raw = await fs.promises.readFile(entryPath, { encoding: "utf8" });

  const result = await sass.compileStringAsync(raw, {
    url: url.pathToFileURL(entryPath),
  });

  const content = result.css.toString();
  const hash = await calcuSha256(content)
  const outputPath = distPath.replace('[contenthash]', hash)
  const outputName = path.relative(Paths["~/root/dist"], outputPath)
  entries.set(entryPath, `/${outputName}`)
  manifest[`/${outputName}`] = outputName

  await fs.promises.mkdir(path.dirname(distPath), { recursive: true });
  await fs.promises.writeFile(outputPath, content, "utf-8");
}

console.log("Copying Assets");
for (const [entryPath, distPath] of assets) {
  await fs.promises.mkdir(path.dirname(distPath), { recursive: true });
  await fs.promises.cp(entryPath, distPath, { recursive: true });
}

console.log("Build HTML");
for (const [entryPath, distPath] of html) {
  const ctx = {
    util: {
      path,
      query: (obj: any) => new URLSearchParams(obj).toString(),
    },
    paths: {
      root: Paths["~/root"],
      dirname: path.dirname(entryPath),
    },
    Paths,
    entries: {
      get: (target: string): string | undefined => {
        if (!path.isAbsolute(target)) {
          target = path.join(path.dirname(entryPath), target)
        }
        target = path.normalize(target)
        return entries.get(target)
      }
    },
    get ctx() {
      return this;
    },
  };

  let result = await ejs.render(
    await fs.promises.readFile(entryPath, "utf8"),
    ctx,
    {
      async: true,
      cache: false,
      filename: entryPath,
    },
  );

  await fs.promises.mkdir(path.dirname(distPath), { recursive: true });
  await fs.promises.writeFile(distPath, result, "utf-8");
}

Object.assign(manifest, await generateManifest(
  Paths["~/root/dist"]
))

await fs.promises.writeFile(Paths["~/root/dist/"]('client_manifest.json'), JSON.stringify(manifest, null, 2),'utf8');

async function calcuSha256(input: string): Promise<string> {
  const msgUint8 = new TextEncoder().encode(input);
  
  const hashBuffer = await crypto.subtle.digest('SHA-256', msgUint8);
  
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');
    
  return hashHex.slice(0, 16);
}

async function generateManifest(
  dirPath: string,
  baseDir: string = dirPath,
  manifest: Record<string, string> = {}
): Promise<Record<string, string>> {
  // Read all entries (files and folders) in the current directory
  const entries = await fs.promises.readdir(dirPath, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = path.join(dirPath, entry.name);
    if (fullPath.endsWith('.html')) {
      continue
    }
    
    const relativePath = path.relative(baseDir, fullPath);
    
    const standardizedPath = relativePath.split(path.sep).join('/');

    if (entry.isDirectory()) {
      await generateManifest(fullPath, baseDir, manifest);
    } else if (entry.isFile()) {
      let manifestKey = `/${standardizedPath}`;

      if (entry.name.endsWith('.html')) {
        if (entry.name === 'index.html') {
          manifestKey = manifestKey.replace(/index\.html$/, ''); 
          if (manifestKey.length > 1 && manifestKey.endsWith('/')) {
            manifestKey = manifestKey.slice(0, -1);
          }
        } else {
          manifestKey = manifestKey.replace(/\.html$/, '');
        }
      }

      manifest[manifestKey] = standardizedPath;
    }
  }

  return manifest;
}