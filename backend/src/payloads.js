const payloads = {
  init: function (name, symbol, base_uri) {
    return {
      name,
      symbol,
      base_uri,
    }
  },
  create: function (
    creator,
    name,
    description,
    number_of_tickets,
    date
  ) {
    return {
      Create: {
        creator: creator,
        name,
        description,
        number_of_tickets,
        date: date,
      }
    }
  },
  hold: function () {
    return {
      Hold: {
      }
    }
  },
  buyTickets: function (amount, metadata) {
    return {
      BuyTickets: {
        amount: amount,
        metadata: metadata,
      }
    }
  }
};

export default payloads;
