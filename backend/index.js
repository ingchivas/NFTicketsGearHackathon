import 'dotenv/config'
import express from "express";
import bodyParser from 'body-parser';

import payloads from './src/payloads.js';
import sendMessage from './src/sendMesage.js';

const app = express();
app.use(bodyParser.json());

app.post('/sendMessage/create', async function (req, res) {
  const { creator, name, description, number_of_tickets, date } = req.body;

  if (!creator || !name || !description || !number_of_tickets || !date) return res.status(400).json({ success: false, message: "A parameter is missing." })

  try {
    const payload = payloads.create(creator, name, description, number_of_tickets, date);
    const result = await sendMessage(payload);

    console.log("sendMessage > ", body, result);

    res.json({ success: true, result });
  } catch (error) {
    console.error(error);
    res.status(500).json({ success: false, message: "Something went wrong internally." });
  }
});

app.post('/sendMessage/buyTickets', async function (req, res) {
  const { amount, metadata } = req.body;

  if (!amount || !metadata) return res.status(400).json({ success: false, message: "A parameter is missing." })

  try {
    const payload = payloads.buyTickets(amount, metadata);
    const result = await sendMessage(payload);

    console.log("sendMessage > ", body, result);

    res.json({ success: true, result });
  } catch (error) {
    console.error(error);
    res.status(500).json({ success: false, message: "Something went wrong internally." });
  }
});

app.listen(3005);
console.log('Express started on port 3005');
