#[cfg(test)]
mod tests {
    use crate::fetchers::{extract_orders, orders_query};
    use serde_json::{from_value, json};
    use graphql_client::Response;

    fn get_orders_response() -> serde_json::Value {
        json!(
            {
                "data": {
                  "shop": {
                    "name": "Frillu"
                  },
                  "orders": {
                    "edges": [
                      {
                        "node": {
                          "name": "#24120",
                          "customer": {
                            "id": "gid://shopify/Customer/2839799398498"
                          },
                          "createdAt": "2020-02-02T21:32:46Z",
                          "updatedAt": "2020-02-02T21:32:47Z",
                          "cancelledAt": null,
                          "shippingAddress": {
                            "address1": "6 elbostan st sheraton airport",
                            "address2": "1",
                            "zip": "11361"
                          },
                          "currentTotalPriceSet": {
                            "shopMoney": {
                              "amount": "376.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "originalTotalPriceSet": {
                            "shopMoney": {
                              "amount": "376.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "fullyPaid": false,
                          "canMarkAsPaid": true,
                          "totalRefundedSet": {
                            "shopMoney": {
                              "amount": "0.0",
                              "currencyCode": "EGP"
                            }
                          }
                        }
                      },
                      {
                        "node": {
                          "name": "#24121",
                          "customer": {
                            "id": "gid://shopify/Customer/2839875649634"
                          },
                          "createdAt": "2020-02-02T22:25:37Z",
                          "updatedAt": "2020-02-02T22:25:37Z",
                          "cancelledAt": null,
                          "shippingAddress": {
                            "address1": "16شارع ارمنت كيلوبترا ",
                            "address2": "شفه11 الدور 6",
                            "zip": "00"
                          },
                          "currentTotalPriceSet": {
                            "shopMoney": {
                              "amount": "787.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "originalTotalPriceSet": {
                            "shopMoney": {
                              "amount": "787.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "fullyPaid": false,
                          "canMarkAsPaid": true,
                          "totalRefundedSet": {
                            "shopMoney": {
                              "amount": "0.0",
                              "currencyCode": "EGP"
                            }
                          }
                        }
                      },
                      {
                        "node": {
                          "name": "#24122",
                          "customer": {
                            "id": "gid://shopify/Customer/2840119869538"
                          },
                          "createdAt": "2020-02-03T03:18:03Z",
                          "updatedAt": "2020-02-03T03:18:04Z",
                          "cancelledAt": null,
                          "shippingAddress": {
                            "address1": "١٢ ش عمرو بن العاص موازي لشارع جمال عبد الناصر ",
                            "address2": "",
                            "zip": "41211"
                          },
                          "currentTotalPriceSet": {
                            "shopMoney": {
                              "amount": "395.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "originalTotalPriceSet": {
                            "shopMoney": {
                              "amount": "395.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "fullyPaid": false,
                          "canMarkAsPaid": true,
                          "totalRefundedSet": {
                            "shopMoney": {
                              "amount": "0.0",
                              "currencyCode": "EGP"
                            }
                          }
                        }
                      },
                      {
                        "node": {
                          "name": "#24123",
                          "customer": {
                            "id": "gid://shopify/Customer/2840185340002"
                          },
                          "createdAt": "2020-02-03T04:49:49Z",
                          "updatedAt": "2020-02-03T04:49:50Z",
                          "cancelledAt": null,
                          "shippingAddress": {
                            "address1": "١٧ ف برج رومانس ةلشطر السابع زهراء المعادي",
                            "address2": "الدور الثامن شقة ٨٣",
                            "zip": "00097"
                          },
                          "currentTotalPriceSet": {
                            "shopMoney": {
                              "amount": "789.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "originalTotalPriceSet": {
                            "shopMoney": {
                              "amount": "789.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "fullyPaid": false,
                          "canMarkAsPaid": true,
                          "totalRefundedSet": {
                            "shopMoney": {
                              "amount": "0.0",
                              "currencyCode": "EGP"
                            }
                          }
                        }
                      },
                      {
                        "node": {
                          "name": "#24124",
                          "customer": {
                            "id": "gid://shopify/Customer/2089166897250"
                          },
                          "createdAt": "2020-02-03T05:39:40Z",
                          "updatedAt": "2020-02-03T05:39:54Z",
                          "cancelledAt": null,
                          "shippingAddress": {
                            "address1": "5 el saraya el kobra square Garden city ",
                            "address2": "Second floor",
                            "zip": "1234"
                          },
                          "currentTotalPriceSet": {
                            "shopMoney": {
                              "amount": "376.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "originalTotalPriceSet": {
                            "shopMoney": {
                              "amount": "376.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "fullyPaid": false,
                          "canMarkAsPaid": true,
                          "totalRefundedSet": {
                            "shopMoney": {
                              "amount": "0.0",
                              "currencyCode": "EGP"
                            }
                          }
                        }
                      },
                      // No shippingAddress, so record should be skipped.
                      {
                        "node": {
                          "name": "#24124",
                          "customer": {
                            "id": "gid://shopify/Customer/2089166897250"
                          },
                          "createdAt": "2020-02-03T05:39:40Z",
                          "updatedAt": "2020-02-03T05:39:54Z",
                          "cancelledAt": null,
                          "shippingAddress": null,
                          "currentTotalPriceSet": {
                            "shopMoney": {
                              "amount": "376.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "originalTotalPriceSet": {
                            "shopMoney": {
                              "amount": "376.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "fullyPaid": false,
                          "canMarkAsPaid": true,
                          "totalRefundedSet": {
                            "shopMoney": {
                              "amount": "0.0",
                              "currencyCode": "EGP"
                            }
                          }
                        }
                      },
                      // No customer, so record should be skipped.
                      {
                        "node": {
                          "name": "#24124",
                          "customer": null,
                          "createdAt": "2020-02-03T05:39:40Z",
                          "updatedAt": "2020-02-03T05:39:54Z",
                          "cancelledAt": null,
                          "shippingAddress": {
                            "address1": "5 el saraya el kobra square Garden city ",
                            "address2": "Second floor",
                            "zip": "1234"
                          },
                          "currentTotalPriceSet": {
                            "shopMoney": {
                              "amount": "376.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "originalTotalPriceSet": {
                            "shopMoney": {
                              "amount": "376.0",
                              "currencyCode": "EGP"
                            }
                          },
                          "fullyPaid": false,
                          "canMarkAsPaid": true,
                          "totalRefundedSet": {
                            "shopMoney": {
                              "amount": "0.0",
                              "currencyCode": "EGP"
                            }
                          }
                        }
                      }
                    ]
                  }
                },
                "extensions": {
                  "cost": {
                    "requestedQueryCost": 33,
                    "actualQueryCost": 33,
                    "throttleStatus": {
                      "maximumAvailable": 1000,
                      "currentlyAvailable": 967,
                      "restoreRate": 50
                    }
                  }
                }
            }
        )
    }

    #[test]
    fn test_parse_gql_orders_response() {
        let resp: Response<orders_query::ResponseData> = from_value(get_orders_response()).unwrap();
        let (orders_vec, invalid) = extract_orders(&resp);
        // One record without shipping address, another without customer.
        assert_eq!(invalid, 2);
        // Even though response contains 7 records, one has no shipping address and another has no customer info,
        // so they're treated as invalid records.
        assert_eq!(orders_vec.len(), 5);
    }
}
