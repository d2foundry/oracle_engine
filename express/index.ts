const express = require("express");
var bodyParser = require("body-parser");
const app = express();
const port: string | number = 6969;
const host: string = "localhost";
let cors = require("cors");
const oracleEngine = require("../pkg");
var jsonParser = bodyParser.json();
app.use(cors());

app.post("/v1/oracle", jsonParser, async function (req: Request, res: any) {
  console.log(req.body);

  if (!req.body) {
    res.status(400).send("Invalid Request");
    return;
  }

  if (
    //@ts-ignore
    !req.body.hash ||
    //@ts-ignore
    !req.body.itemFamily ||
    //@ts-ignore
    !req.body.itemSubFamily ||
    //@ts-ignore
    !req.body.ammoType ||
    //@ts-ignore
    !req.body.damageType ||
    //@ts-ignore
    !req.body.stats
  ) {
    res.status(400).send("Invalid Request");
    return;
  }
  const statMap = new Map();
  //@ts-ignore
  for (const [key, value] of Object.entries(req.body.stats)) {
    statMap.set(parseInt(key), value);
  }
  console.log(req.body);
  oracleEngine.setWeapon(
    //@ts-ignore
    parseInt(req.body.hash),
    //@ts-ignore
    parseInt(req.body.itemFamily),
    //@ts-ignore
    parseInt(req.body.itemSubFamily),
    //@ts-ignore
    parseInt(req.body.ammoType),
    //@ts-ignore
    parseInt(req.body.damageType),
  );
  oracleEngine.setStats(statMap);
  let item = oracleEngine.stringifyWeapon();
  console.log(item);
  res.status(200).send(JSON.stringify(item));
});

app.listen(port, () => {
  console.log(`Oracle Engine Server running on http://localhost:${port}`);
  console.log(
    `This server is **not** intended for public use and should only be run in a local environment (i.e localhost)`,
  );
});
