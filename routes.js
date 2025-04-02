const express = require("express");
const axios = require("axios");
const { exec } = require("child_process");

const router = express.Router();

require("dotenv").config();


router.get("/", (req, res) => {
  res.send("Welcome to the Webhooks API");
});

// router.post("/webhook", async (req, res) => {
//   const payload = req.body;

//   if (payload.action === "closed" && payload.pull_request.merged) {
//     console.log(`Pull request #${payload.pull_request.number} merged into main by ${payload.sender.login}`);

//     const githubOwner = "ayushka11"; // Your GitHub username or org
//     const githubRepo = "test"; // Your repository name
//     const githubToken = process.env.GITHUB_TOKEN; // GitHub token from env vars

//     try {
//       // Step 1: Get the last two commits from the build branch
//       const commitsUrl = `https://api.github.com/repos/${githubOwner}/${githubRepo}/commits?sha=build&per_page=2`;
//       const commitResponse = await axios.get(commitsUrl, {
//         headers: { Authorization: `token ${githubToken}` },
//       });

//       if (commitResponse.data.length < 2) {
//         console.log("Not enough commits in build branch.");
//         return res.status(400).json({ error: "Not enough commits to compare." });
//       }

//       const mergeCommit = commitResponse.data[0].sha; // Latest commit
//       const baseCommit = commitResponse.data[1].sha; // Previous commit

//       console.log(`Merge commit: ${mergeCommit}`);
//       console.log(`Base commit: ${baseCommit}`);

//       // Step 2: Fetch the diff between these two commits
//       const diffUrl = `https://api.github.com/repos/${githubOwner}/${githubRepo}/compare/${baseCommit}...${mergeCommit}`;
//       const diffResponse = await axios.get(diffUrl, {
//         headers: {
//           Authorization: `token ${githubToken}`,
//           Accept: "application/vnd.github.v3.diff", // Get the diff output
//         },
//       });

//       console.log("Received PR Diff:\n", diffResponse.data);

//       const diffData = diffResponse.data;
//       const regex = /access\/([^\/]+)\/([^\/]+)\/([\w\d]+)/;
//       const match = diffData.match(regex);

//       const [, project, cloudProvider, hash] = match;
//       console.log("Project:", project);
//       console.log("Cloud Provider:", cloudProvider);
//       console.log("Hash:", hash);

//       const filePath = `names/${hash}`;
//       const getNameUrl = `https://api.github.com/repos/${githubOwner}/${githubRepo}/contents/${filePath}?ref=build`;

//       try {
//         const nameResponse = await axios.get(getNameUrl, {
//           headers: {
//             Authorization: `Bearer ${githubToken}`, // 🔄 Use Bearer instead of token
//             Accept: 'application/vnd.github.v3+json'
//           }
//         });

//         if (nameResponse.data) {
//           // File content is base64 encoded
//           const base64Content = nameResponse.data.content;
//           const decodedContent = Buffer.from(base64Content, 'base64').toString('utf-8');
    
//           console.log("Decoded File Content:", decodedContent);
//         } else {
//           console.log("No data found for the given hash.");
//         }
//       } catch (error) {
//         console.error("Error fetching file:", error.response?.data || error.message);
//       }

//       return res.status(200).json({ message: "Diff successfully fetched and sent." });
//     } catch (error) {
//       console.error("Error fetching Git diff:", error.response ? error.response.data : error.message);
//       return res.status(500).json({ error: "Internal Server Error" });
//     }
//   }

//   res.status(200).send({ message: "No action taken" });
// });

router.post("/webhook", async (req, res) => {
  console.log("Running in test mode...");
  const testProject = "testProject";
  const testCloudProvider = "aws";
  const testHash = "testHash";
  const testUser = "iris";
  const testGroup = "ayushka";
  console.log("Test Project:", testProject);
  console.log("Test Cloud Provider:", testCloudProvider);
  console.log("Test Hash:", testHash);
  // Manually call function to add user to group
  addUserToGroup(testUser, testGroup);
  return res.status(200).json({ message: "Test data processed successfully." });
});

function addUserToGroup(user, group) {
  const cmd = `sudo usermod -aG ${group} ${user}`;
  
  exec(cmd, (error, stdout, stderr) => {
    if (error) {
      console.error(`Error adding user: ${error.message}`);
      return;
    }
    if (stderr) {
      console.error(`stderr: ${stderr}`);
      return;
    }
    console.log(`User ${user} added to ${group}: ${stdout}`);
  });
}

module.exports = router;
