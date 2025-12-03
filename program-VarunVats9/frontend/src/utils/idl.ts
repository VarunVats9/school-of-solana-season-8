/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/tipping.json`.
 */
export type Tipping = {
    "address": "CeA7jNGCbhQvhAcWPceXNQtf13wm3oNiFtfD6tdU92PV",
    "metadata": {
        "name": "tipping",
        "version": "0.1.0",
        "spec": "0.1.0",
        "description": "Created with Anchor"
    },
    "instructions": [
        {
            "name": "initializeUser",
            "discriminator": [
                111,
                17,
                185,
                250,
                60,
                122,
                38,
                254
            ],
            "accounts": [
                {
                    "name": "signer",
                    "writable": true,
                    "signer": true
                },
                {
                    "name": "userStats",
                    "writable": true,
                    "pda": {
                        "seeds": [
                            {
                                "kind": "const",
                                "value": [
                                    117,
                                    115,
                                    101,
                                    114,
                                    45,
                                    115,
                                    116,
                                    97,
                                    116,
                                    115
                                ]
                            },
                            {
                                "kind": "account",
                                "path": "signer"
                            }
                        ]
                    }
                },
                {
                    "name": "systemProgram",
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": []
        },
        {
            "name": "tip",
            "discriminator": [
                77,
                164,
                35,
                21,
                36,
                121,
                213,
                51
            ],
            "accounts": [
                {
                    "name": "signer",
                    "writable": true,
                    "signer": true
                },
                {
                    "name": "recipient",
                    "writable": true
                },
                {
                    "name": "userStats",
                    "writable": true,
                    "pda": {
                        "seeds": [
                            {
                                "kind": "const",
                                "value": [
                                    117,
                                    115,
                                    101,
                                    114,
                                    45,
                                    115,
                                    116,
                                    97,
                                    116,
                                    115
                                ]
                            },
                            {
                                "kind": "account",
                                "path": "signer"
                            }
                        ]
                    }
                },
                {
                    "name": "systemProgram",
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": [
                {
                    "name": "amount",
                    "type": "u64"
                }
            ]
        }
    ],
    "accounts": [
        {
            "name": "userStats",
            "discriminator": [
                176,
                223,
                136,
                27,
                122,
                79,
                32,
                227
            ]
        }
    ],
    "types": [
        {
            "name": "userStats",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "totalTipsSent",
                        "type": "u64"
                    },
                    {
                        "name": "totalTipsReceived",
                        "type": "u64"
                    },
                    {
                        "name": "bump",
                        "type": "u8"
                    }
                ]
            }
        }
    ]
};

export const IDL: Tipping = {
    "address": "CeA7jNGCbhQvhAcWPceXNQtf13wm3oNiFtfD6tdU92PV",
    "metadata": {
        "name": "tipping",
        "version": "0.1.0",
        "spec": "0.1.0",
        "description": "Created with Anchor"
    },
    "instructions": [
        {
            "name": "initializeUser",
            "discriminator": [
                111,
                17,
                185,
                250,
                60,
                122,
                38,
                254
            ],
            "accounts": [
                {
                    "name": "signer",
                    "writable": true,
                    "signer": true
                },
                {
                    "name": "userStats",
                    "writable": true,
                    "pda": {
                        "seeds": [
                            {
                                "kind": "const",
                                "value": [
                                    117,
                                    115,
                                    101,
                                    114,
                                    45,
                                    115,
                                    116,
                                    97,
                                    116,
                                    115
                                ]
                            },
                            {
                                "kind": "account",
                                "path": "signer"
                            }
                        ]
                    }
                },
                {
                    "name": "systemProgram",
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": []
        },
        {
            "name": "tip",
            "discriminator": [
                77,
                164,
                35,
                21,
                36,
                121,
                213,
                51
            ],
            "accounts": [
                {
                    "name": "signer",
                    "writable": true,
                    "signer": true
                },
                {
                    "name": "recipient",
                    "writable": true
                },
                {
                    "name": "userStats",
                    "writable": true,
                    "pda": {
                        "seeds": [
                            {
                                "kind": "const",
                                "value": [
                                    117,
                                    115,
                                    101,
                                    114,
                                    45,
                                    115,
                                    116,
                                    97,
                                    116,
                                    115
                                ]
                            },
                            {
                                "kind": "account",
                                "path": "signer"
                            }
                        ]
                    }
                },
                {
                    "name": "systemProgram",
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": [
                {
                    "name": "amount",
                    "type": "u64"
                }
            ]
        }
    ],
    "accounts": [
        {
            "name": "userStats",
            "discriminator": [
                176,
                223,
                136,
                27,
                122,
                79,
                32,
                227
            ]
        }
    ],
    "types": [
        {
            "name": "userStats",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "totalTipsSent",
                        "type": "u64"
                    },
                    {
                        "name": "totalTipsReceived",
                        "type": "u64"
                    },
                    {
                        "name": "bump",
                        "type": "u8"
                    }
                ]
            }
        }
    ]
};
