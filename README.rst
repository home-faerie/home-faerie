Home Daemon
===========

Home Daemon is a set of lightweight utilities for logging
data from various sensors and meters.

Features
--------

* Zigbee2MQTT sensor data logging to PostgreSQL database

Setup
-----

Create database schema:

.. code-block:: sql

    CREATE TABLE device_meters (
        "timestamp" timestamp with time zone,
        name text,
        value double precision,
        dimensions jsonb,
        value_meta json
    );

And run the application:

.. code-block:: text

    $ POSTGRESQL_URL=postgresql:/meters MQTT_URL=zigbee2mqtt-gateway.local MQTT_PORT=1883 cargo run