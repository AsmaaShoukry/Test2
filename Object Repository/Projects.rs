<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Projects</name>
   <tag></tag>
   <elementGuidId>df8fb45f-fab6-4ce1-9669-b7b647ecf91c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6Ijc5MDY5ODk3ODU2YzY2NGY3MWM4OTRkYjc0ZDM5OWY1ZDJkOWU5M2NiNDE5MjRkNWFiM2I1MjFiMjFhNWQ2ODk4Mjk0OTY2MzMzOTM0YjE3In0.eyJhdWQiOiIxIiwianRpIjoiNzkwNjk4OTc4NTZjNjY0ZjcxYzg5NGRiNzRkMzk5ZjVkMmQ5ZTkzY2I0MTkyNGQ1YWIzYjUyMWIyMWE1ZDY4OTgyOTQ5NjYzMzM5MzRiMTciLCJpYXQiOjE1NTEwODQyNTMsIm5iZiI6MTU1MTA4NDI1MywiZXhwIjoxNTgyNjIwMjUzLCJzdWIiOiI1NSIsInNjb3BlcyI6W119.lxRDbDgCGgDqz9v9DPiqlYd6lajtYPi5P-LKiJeu2azuzmFpmTIavE4-tE5fJ-UjTIUoZLxb-_pRq-dYuJGZxT23fM6aRQ5LIu9EgRXbOQWtORUYk2Y_tDqX9S5kFyci474ckcFO4MOJc2FBXBkOj6SXoMookp0FByGFzhZ0SzzxBJ-xBCoDn48BpX8I7vgk-4A1I1BayP8VDENhHQTJTqnydutFgB3UX7YaZtyjsuUiVwb99nqo0YsK9kQo0TdospLfz5naGYEBgBQq0k0ZB5Qs21TRlX5YbBO5AqIzdWK-RuJIRta2javIohQFVJqyoWJrHIoIKJnWrLscirg_ZKNhxg5yx2NDDnXv-Z7LENTI9FU2v4yyxaJCNsPcbcEy_4Kz4btHBbM7qDC4XfjEKQbtJMJ2ouVklPKEPCyLsppLf7RPdPBuop-Fw5Hb_g83auLY4wSDu_XvPXzMhQ91MTiAFU74rPo_xCoujijOktqLiJAwai1eHpcFVYDarX7s4wF8xmNly54okvqyyYuA1cMWw4OiLioxQpwYt2wy44tmqEr6VR2Drdj7BBTHJdzHpRqP6M46lLq5XDxpGX47NjLO3kgc_Egf93otNW_CO6m2nHMwAniOJ6PmYsaQI3UTvZt-nLO-BNHUDYIPUUh5TUMwrmTqPAQ3djvJC3RNcGs</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://shikaty-core.dev.brightcreations.com/api/v1/projects?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, 'data[0].name', &quot;Project1&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
