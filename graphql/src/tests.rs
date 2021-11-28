#[cfg(test)]
mod tests {
    use crate::fetchers::{extract_orders, extract_inventory, orders_query, inventory_query};
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

    fn get_inventory_response() -> serde_json::Value {
      json!(
        {
          "data": {
            "shop": {
              "name": "Frillu"
            },
            "locations": {
              "edges": [
                {
                  "node": {
                    "id": "gid://shopify/Location/304939036",
                    "name": "Osman Ahmed Osman Bridge",
                    "inventoryLevels": {
                      "edges": [
                        {
                          "node": {
                            "id": "gid://shopify/InventoryLevel/2237456?inventory_item_id=32808904851554",
                            "item": {
                              "id": "gid://shopify/InventoryItem/32808904851554",
                              "variant": {
                                "displayName": "Rocker Open Sweat Cardi - Default Title",
                                "price": "700.00",
                                "inventoryQuantity": 3
                              }
                            },
                            "createdAt": "2020-02-11T17:53:33Z"
                          }
                        },
                        {
                          "node": {
                            "id": "gid://shopify/InventoryLevel/2237456?inventory_item_id=32850182275170",
                            "item": {
                              "id": "gid://shopify/InventoryItem/32850182275170",
                              "variant": {
                                "displayName": "Olive Urban Fly Shirt - MEDIUM",
                                "price": "500.00",
                                "inventoryQuantity": 0
                              }
                            },
                            "createdAt": "2020-02-22T18:30:50Z"
                          }
                        },
                        {
                          "node": {
                            "id": "gid://shopify/InventoryLevel/2237456?inventory_item_id=32850182307938",
                            "item": {
                              "id": "gid://shopify/InventoryItem/32850182307938",
                              "variant": {
                                "displayName": "Olive Urban Fly Shirt - LARGE",
                                "price": "500.00",
                                "inventoryQuantity": 0
                              }
                            },
                            "createdAt": "2020-02-22T18:30:50Z"
                          }
                        },
                        {
                          "node": {
                            "id": "gid://shopify/InventoryLevel/2237456?inventory_item_id=32850182340706",
                            "item": {
                              "id": "gid://shopify/InventoryItem/32850182340706",
                              "variant": {
                                "displayName": "Olive Urban Fly Shirt - X-LARGE",
                                "price": "500.00",
                                "inventoryQuantity": 0
                              }
                            },
                            "createdAt": "2020-02-22T18:30:50Z"
                          }
                        },
                        {
                          "node": {
                            "id": "gid://shopify/InventoryLevel/2237456?inventory_item_id=32850189877346",
                            "item": {
                              "id": "gid://shopify/InventoryItem/32850189877346",
                              "variant": {
                                "displayName": "Purple Urban Fly Shirt - MEDIUM",
                                "price": "500.00",
                                "inventoryQuantity": 9
                              }
                            },
                            "createdAt": "2020-02-22T18:36:13Z"
                          }
                        }
                      ]
                    }
                  }
                }
              ]
            }
          },
          "extensions": {
            "cost": {
              "requestedQueryCost": 21,
              "actualQueryCost": 21,
              "throttleStatus": {
                "maximumAvailable": 1000,
                "currentlyAvailable": 979,
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

    #[test]
    fn test_parse_gql_inventory_response() {
      let resp: Response<inventory_query::ResponseData> = from_value(get_inventory_response()).unwrap();
        let (inventory_vec, _) = extract_inventory(&resp);
        assert_eq!(inventory_vec.len(), 5);
        assert_eq!(inventory_vec[0].item.quantity, 3);
        assert_eq!(inventory_vec[0].item.id, String::from("gid://shopify/InventoryItem/32808904851554"));
        assert_eq!(inventory_vec[inventory_vec.len() - 1].item.display_name, String::from("Purple Urban Fly Shirt - MEDIUM"));
        assert_eq!(inventory_vec[inventory_vec.len() - 1].item.price.amount, 500 as f32);
    }
}
