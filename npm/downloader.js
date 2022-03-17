/**
 * This file Copyright (c) 2010-2022 Magnolia International
 * Ltd.  (http://www.magnolia-cms.com). All rights reserved.
 *
 *
 * This program and the accompanying materials are made
 * available under the terms of the Magnolia Network Agreement
 * which accompanies this distribution, and is available at
 * http://www.magnolia-cms.com/mna.html
 *
 * Any modifications to this file must keep this entire header
 * intact.
 *
 */
const axios = require("axios");
const tar = require("tar");
const { spawnSync } = require("child_process");
const fs = require("fs");
const path = require("path")

class Downloader {
    constructor(url, options) {
        this.url = url;
        this.options = options;
    }
    async install() {
        const stream = await axios({ url: this.url, responseType: "stream" });

        if (!fs.existsSync(this.options.installDirectory))
            fs.mkdirSync(this.options.installDirectory);
        stream.data.pipe(tar.x({ C: this.options.installDirectory }))

    }

    binary_path() {
        return path.join(this.options.installDirectory, this.options.name)
    }

    run() {
        const options = { cwd: process.cwd(), stdio: "inherit" };
        const [, , ...args] = process.argv;
        const result = spawnSync(this.binary_path(), args, options);

        if (result.error) {
            console.log(result.error)
        }

        process.exit(result.status);
    }
}

module.exports = Downloader;