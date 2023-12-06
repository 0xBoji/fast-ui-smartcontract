# FastUI-smartcontract
Build contract fastUI
near call s_contract add_message '{"message":"","widget_id":"","widget_name":"","account_give_star":"","star":}' --accountId your_accid_calling
near view s_contract get_messages '{"widget_id":""}'

ex output:
abc.near said "hello near"

near view s_contract get_star '{"widget_id":"abc"}'

ex output:
neardevhub - 5
