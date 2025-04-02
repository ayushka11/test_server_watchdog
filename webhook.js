const express = require("express");
const routes = require("./routes");
const bodyParser = require('body-parser');
const crypto = require('crypto');

// App
const app = express();

const sigHeaderName = 'X-Hub-Signature-256';
const sigHashAlg = 'sha256';
const secret = "ABCD1234";


app.use(bodyParser.json(
    {
        verify: (req, res, buf, encoding) => {
            if (buf && buf.length) {
            req.rawBody = buf.toString(encoding || 'utf8');
            }
        },
    }
));

// Set port
const port = process.env.PORT || "2000";
app.set("port", port);

app.use('/', routes);

// Server
app.listen(port, () => console.log(`Server running on localhost:${port}`));