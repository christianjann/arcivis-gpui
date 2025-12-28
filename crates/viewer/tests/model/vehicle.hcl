# Vehicle System Model in HCL/Terraform DSL

# Package definition
package "VehicleSystem" {
  description = "A comprehensive vehicle system model"
  version     = "1.0.0"
}

# Abstract part definitions
part "Vehicle" {
  description = "Base vehicle definition"
  
  attribute "mass" {
    type    = "Real"
    unit    = "kg"
    default = 1500.0
  }
  
  attribute "maxSpeed" {
    type    = "Real"
    unit    = "km/h"
    default = 200.0
  }
  
  attribute "vin" {
    type = "String"
  }

  # Nested parts
  part "powertrain" {
    type = "Powertrain"
  }
  
  part "chassis" {
    type = "Chassis"
  }
  
  part "body" {
    type = "Body"
  }
  
  part "electricalSystem" {
    type = "ElectricalSystem"
  }
}

# Powertrain subsystem
part "Powertrain" {
  description = "Vehicle powertrain system"
  
  attribute "power" {
    type    = "Real"
    unit    = "kW"
    default = 150.0
  }
  
  attribute "torque" {
    type    = "Real"
    unit    = "Nm"
    default = 320.0
  }

  part "engine" {
    type = "Engine"
  }
  
  part "transmission" {
    type = "Transmission"
  }
  
  part "driveshaft" {
    type = "Driveshaft"
  }
}

part "Engine" {
  description = "Internal combustion or electric motor"
  
  attribute "displacement" {
    type    = "Real"
    unit    = "L"
    default = 2.0
  }
  
  attribute "cylinders" {
    type    = "Integer"
    default = 4
  }
  
  attribute "fuelType" {
    type    = "String"
    default = "gasoline"
    enum    = ["gasoline", "diesel", "electric", "hybrid"]
  }

  port "mechanicalOut" {
    direction = "out"
    type      = "MechanicalPower"
  }
  
  port "fuelIn" {
    direction = "in"
    type      = "FuelFlow"
  }
}

part "Transmission" {
  description = "Gearbox and clutch assembly"
  
  attribute "gears" {
    type    = "Integer"
    default = 6
  }
  
  attribute "type" {
    type    = "String"
    default = "automatic"
    enum    = ["manual", "automatic", "cvt", "dct"]
  }

  port "mechanicalIn" {
    direction = "in"
    type      = "MechanicalPower"
  }
  
  port "mechanicalOut" {
    direction = "out"
    type      = "MechanicalPower"
  }
}

part "Driveshaft" {
  description = "Power transfer to wheels"
  
  port "mechanicalIn" {
    direction = "in"
    type      = "MechanicalPower"
  }
  
  port "wheelOut" {
    direction = "out"
    type      = "MechanicalPower"
    multiplicity = 4
  }
}

# Chassis subsystem
part "Chassis" {
  description = "Vehicle chassis and suspension"
  
  attribute "wheelbase" {
    type    = "Real"
    unit    = "mm"
    default = 2700.0
  }
  
  attribute "trackWidth" {
    type    = "Real"
    unit    = "mm"
    default = 1550.0
  }

  part "frontSuspension" {
    type = "Suspension"
  }
  
  part "rearSuspension" {
    type = "Suspension"
  }
  
  part "wheels" {
    type         = "Wheel"
    multiplicity = 4
  }
  
  part "brakes" {
    type = "BrakeSystem"
  }
  
  part "steering" {
    type = "SteeringSystem"
  }
}

part "Suspension" {
  description = "Suspension assembly"
  
  attribute "type" {
    type    = "String"
    default = "macpherson"
    enum    = ["macpherson", "double_wishbone", "multi_link", "solid_axle"]
  }
  
  attribute "springRate" {
    type    = "Real"
    unit    = "N/mm"
    default = 35.0
  }
  
  attribute "damperRate" {
    type    = "Real"
    unit    = "Ns/mm"
    default = 3.0
  }
}

part "Wheel" {
  description = "Wheel and tire assembly"
  
  attribute "diameter" {
    type    = "Real"
    unit    = "inch"
    default = 17.0
  }
  
  attribute "width" {
    type    = "Real"
    unit    = "mm"
    default = 225.0
  }
  
  attribute "tireProfile" {
    type    = "Integer"
    default = 45
  }

  port "mechanicalIn" {
    direction = "in"
    type      = "MechanicalPower"
  }
  
  port "groundContact" {
    direction = "inout"
    type      = "GroundForce"
  }
}

part "BrakeSystem" {
  description = "Vehicle braking system"
  
  attribute "type" {
    type    = "String"
    default = "disc"
    enum    = ["disc", "drum", "regenerative"]
  }
  
  attribute "absEnabled" {
    type    = "Boolean"
    default = true
  }

  part "frontBrakes" {
    type         = "BrakeCaliper"
    multiplicity = 2
  }
  
  part "rearBrakes" {
    type         = "BrakeCaliper"
    multiplicity = 2
  }
  
  part "absController" {
    type = "ABSController"
  }
}

part "BrakeCaliper" {
  attribute "pistons" {
    type    = "Integer"
    default = 4
  }
  
  attribute "rotorDiameter" {
    type    = "Real"
    unit    = "mm"
    default = 330.0
  }
}

part "ABSController" {
  port "wheelSpeedIn" {
    direction    = "in"
    type         = "WheelSpeed"
    multiplicity = 4
  }
  
  port "brakeControlOut" {
    direction    = "out"
    type         = "BrakeCommand"
    multiplicity = 4
  }
}

part "SteeringSystem" {
  description = "Power steering system"
  
  attribute "type" {
    type    = "String"
    default = "electric"
    enum    = ["hydraulic", "electric", "electro_hydraulic"]
  }
  
  attribute "ratio" {
    type    = "Real"
    default = 15.5
  }

  port "steeringInput" {
    direction = "in"
    type      = "SteeringAngle"
  }
  
  port "wheelAngleOut" {
    direction    = "out"
    type         = "WheelAngle"
    multiplicity = 2
  }
}

# Body subsystem
part "Body" {
  description = "Vehicle body and interior"
  
  attribute "doors" {
    type    = "Integer"
    default = 4
  }
  
  attribute "seats" {
    type    = "Integer"
    default = 5
  }
  
  attribute "bodyStyle" {
    type    = "String"
    default = "sedan"
    enum    = ["sedan", "coupe", "hatchback", "suv", "wagon", "convertible"]
  }

  part "exterior" {
    type = "Exterior"
  }
  
  part "interior" {
    type = "Interior"
  }
}

part "Exterior" {
  part "frontBumper" {
    type = "Bumper"
  }
  
  part "rearBumper" {
    type = "Bumper"
  }
  
  part "headlights" {
    type         = "Light"
    multiplicity = 2
  }
  
  part "taillights" {
    type         = "Light"
    multiplicity = 2
  }
  
  part "mirrors" {
    type         = "Mirror"
    multiplicity = 2
  }
}

part "Bumper" {
  attribute "material" {
    type    = "String"
    default = "plastic"
  }
}

part "Light" {
  attribute "type" {
    type    = "String"
    default = "led"
    enum    = ["halogen", "xenon", "led", "laser"]
  }
  
  attribute "power" {
    type    = "Real"
    unit    = "W"
    default = 35.0
  }
}

part "Mirror" {
  attribute "heated" {
    type    = "Boolean"
    default = true
  }
  
  attribute "autoFolding" {
    type    = "Boolean"
    default = false
  }
}

part "Interior" {
  part "dashboard" {
    type = "Dashboard"
  }
  
  part "seats" {
    type         = "Seat"
    multiplicity = 5
  }
  
  part "hvac" {
    type = "HVACSystem"
  }
}

part "Dashboard" {
  part "instrumentCluster" {
    type = "InstrumentCluster"
  }
  
  part "infotainment" {
    type = "InfotainmentSystem"
  }
}

part "InstrumentCluster" {
  attribute "type" {
    type    = "String"
    default = "digital"
    enum    = ["analog", "digital", "hybrid"]
  }
  
  attribute "screenSize" {
    type    = "Real"
    unit    = "inch"
    default = 12.3
  }
}

part "InfotainmentSystem" {
  attribute "screenSize" {
    type    = "Real"
    unit    = "inch"
    default = 10.0
  }
  
  attribute "appleCarPlay" {
    type    = "Boolean"
    default = true
  }
  
  attribute "androidAuto" {
    type    = "Boolean"
    default = true
  }

  port "audioOut" {
    direction = "out"
    type      = "AudioSignal"
  }
  
  port "canBus" {
    direction = "inout"
    type      = "CANBus"
  }
}

part "Seat" {
  attribute "material" {
    type    = "String"
    default = "leather"
    enum    = ["cloth", "leather", "alcantara", "vinyl"]
  }
  
  attribute "heated" {
    type    = "Boolean"
    default = true
  }
  
  attribute "ventilated" {
    type    = "Boolean"
    default = false
  }
  
  attribute "powerAdjust" {
    type    = "Boolean"
    default = true
  }
}

part "HVACSystem" {
  attribute "zones" {
    type    = "Integer"
    default = 2
  }
  
  attribute "autoClimate" {
    type    = "Boolean"
    default = true
  }

  port "temperatureIn" {
    direction = "in"
    type      = "Temperature"
  }
  
  port "airflowOut" {
    direction = "out"
    type      = "Airflow"
  }
}

# Electrical subsystem
part "ElectricalSystem" {
  description = "Vehicle electrical and electronic systems"
  
  attribute "voltage" {
    type    = "Real"
    unit    = "V"
    default = 12.0
  }

  part "battery" {
    type = "Battery"
  }
  
  part "alternator" {
    type = "Alternator"
  }
  
  part "wiring" {
    type = "WiringHarness"
  }
  
  part "ecu" {
    type = "ECU"
  }
  
  part "sensors" {
    type = "SensorArray"
  }
}

part "Battery" {
  attribute "capacity" {
    type    = "Real"
    unit    = "Ah"
    default = 70.0
  }
  
  attribute "type" {
    type    = "String"
    default = "lead_acid"
    enum    = ["lead_acid", "agm", "lithium"]
  }

  port "powerOut" {
    direction = "out"
    type      = "ElectricalPower"
  }
}

part "Alternator" {
  attribute "output" {
    type    = "Real"
    unit    = "A"
    default = 150.0
  }

  port "mechanicalIn" {
    direction = "in"
    type      = "MechanicalPower"
  }
  
  port "powerOut" {
    direction = "out"
    type      = "ElectricalPower"
  }
}

part "WiringHarness" {
  attribute "totalLength" {
    type    = "Real"
    unit    = "m"
    default = 2500.0
  }
}

part "ECU" {
  description = "Engine Control Unit"
  
  attribute "processorSpeed" {
    type    = "Real"
    unit    = "MHz"
    default = 200.0
  }
  
  attribute "memory" {
    type    = "Real"
    unit    = "MB"
    default = 4.0
  }

  port "sensorInputs" {
    direction = "in"
    type      = "SensorData"
  }
  
  port "actuatorOutputs" {
    direction = "out"
    type      = "ActuatorCommand"
  }
  
  port "canBus" {
    direction = "inout"
    type      = "CANBus"
  }
}

part "SensorArray" {
  part "temperatureSensors" {
    type         = "TemperatureSensor"
    multiplicity = 8
  }
  
  part "pressureSensors" {
    type         = "PressureSensor"
    multiplicity = 6
  }
  
  part "speedSensors" {
    type         = "SpeedSensor"
    multiplicity = 4
  }
  
  part "o2Sensors" {
    type         = "O2Sensor"
    multiplicity = 2
  }
}

part "TemperatureSensor" {
  attribute "range" {
    type    = "String"
    default = "-40C to 150C"
  }

  port "dataOut" {
    direction = "out"
    type      = "Temperature"
  }
}

part "PressureSensor" {
  attribute "range" {
    type    = "String"
    default = "0 to 10 bar"
  }

  port "dataOut" {
    direction = "out"
    type      = "Pressure"
  }
}

part "SpeedSensor" {
  port "dataOut" {
    direction = "out"
    type      = "WheelSpeed"
  }
}

part "O2Sensor" {
  attribute "type" {
    type    = "String"
    default = "wideband"
    enum    = ["narrowband", "wideband"]
  }

  port "dataOut" {
    direction = "out"
    type      = "O2Level"
  }
}

# Connections between parts
connection "engine_to_transmission" {
  from = "Vehicle.powertrain.engine.mechanicalOut"
  to   = "Vehicle.powertrain.transmission.mechanicalIn"
}

connection "transmission_to_driveshaft" {
  from = "Vehicle.powertrain.transmission.mechanicalOut"
  to   = "Vehicle.powertrain.driveshaft.mechanicalIn"
}

connection "driveshaft_to_wheels" {
  from = "Vehicle.powertrain.driveshaft.wheelOut"
  to   = "Vehicle.chassis.wheels.mechanicalIn"
}

connection "speed_sensors_to_abs" {
  from = "Vehicle.electricalSystem.sensors.speedSensors.dataOut"
  to   = "Vehicle.chassis.brakes.absController.wheelSpeedIn"
}

connection "ecu_to_can" {
  from = "Vehicle.electricalSystem.ecu.canBus"
  to   = "Vehicle.body.interior.dashboard.infotainment.canBus"
}

# Requirements
requirement "performance" {
  id          = "REQ-001"
  description = "Vehicle shall achieve 0-100 km/h in under 8 seconds"
  
  constraint "acceleration" {
    expression = "acceleration_time < 8.0"
    unit       = "s"
  }
}

requirement "fuel_efficiency" {
  id          = "REQ-002"
  description = "Vehicle shall achieve minimum fuel efficiency"
  
  constraint "consumption" {
    expression = "fuel_consumption < 8.0"
    unit       = "L/100km"
  }
}

requirement "safety" {
  id          = "REQ-003"
  description = "Vehicle shall meet safety standards"
  
  constraint "crash_rating" {
    expression = "ncap_rating >= 5"
  }
}

# Analysis cases
analysis "performance_analysis" {
  description = "Analyze vehicle performance characteristics"
  
  input "vehicle_mass" {
    from = "Vehicle.mass"
  }
  
  input "engine_power" {
    from = "Vehicle.powertrain.power"
  }
  
  output "power_to_weight" {
    expression = "engine_power / vehicle_mass"
    unit       = "kW/kg"
  }
  
  output "estimated_0_100" {
    expression = "calculate_acceleration(vehicle_mass, engine_power)"
    unit       = "s"
  }
}

analysis "weight_distribution" {
  description = "Calculate front/rear weight distribution"
  
  output "front_weight_percent" {
    expression = "front_axle_load / total_weight * 100"
    unit       = "%"
  }
  
  output "rear_weight_percent" {
    expression = "rear_axle_load / total_weight * 100"
    unit       = "%"
  }
}
